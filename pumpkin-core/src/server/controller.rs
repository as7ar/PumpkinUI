use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crate::config::{load_or_default, save_config};
use crate::error::{PumpkinError, Result};
use crate::model::{PumpkinConfig, ServerLogLine, ServerSnapshot, ServerStatus, ServerStream};
use crate::server::download::ensure_executable;
use crate::util::resolve_path;

const LOG_LIMIT: usize = 512;
const STOP_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug, Clone)]
pub struct ServerController {
    inner: Arc<Mutex<ServerControllerInner>>,
}

#[derive(Debug)]
struct ServerControllerInner {
    config_path: PathBuf,
    config: PumpkinConfig,
    status: ServerStatus,
    process: Option<ServerProcess>,
    logs: VecDeque<ServerLogLine>,
    last_error: Option<String>,
    active_pid: Option<u32>,
}

#[derive(Debug)]
struct ServerProcess {
    pid: u32,
    child: Arc<Mutex<Child>>,
    stdin: Arc<Mutex<ChildStdin>>,
}

impl ServerController {
    pub fn new(config_path: impl Into<PathBuf>, config: PumpkinConfig) -> Self {
        Self {
            inner: Arc::new(Mutex::new(ServerControllerInner {
                config_path: config_path.into(),
                config,
                status: ServerStatus::Stopped,
                process: None,
                logs: VecDeque::new(),
                last_error: None,
                active_pid: None,
            })),
        }
    }

    pub fn from_config_path(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let config = load_or_default(&path)?;
        Ok(Self::new(path, config))
    }

    pub fn config(&self) -> PumpkinConfig {
        let inner = self.inner.lock().expect("server controller poisoned");
        inner.config.clone()
    }

    pub fn set_config(&self, config: PumpkinConfig) {
        let mut inner = self.inner.lock().expect("server controller poisoned");
        inner.config = config;
    }

    pub fn config_path(&self) -> PathBuf {
        let inner = self.inner.lock().expect("server controller poisoned");
        inner.config_path.clone()
    }

    pub fn save_config(&self) -> Result<()> {
        let inner = self.inner.lock().expect("server controller poisoned");
        save_config(&inner.config_path, &inner.config)
    }

    pub fn status(&self) -> ServerSnapshot {
        let inner = self.inner.lock().expect("server controller poisoned");
        ServerSnapshot {
            status: inner.status,
            process_id: inner.active_pid,
            last_error: inner.last_error.clone(),
        }
    }

    pub fn logs(&self) -> Vec<ServerLogLine> {
        let inner = self.inner.lock().expect("server controller poisoned");
        inner.logs.iter().cloned().collect()
    }

    pub fn drain_logs(&self) -> Vec<String> {
        let mut inner = self.inner.lock().expect("server controller poisoned");
        inner.logs.drain(..).map(|line| line.render()).collect()
    }

    pub fn start(&self) -> Result<()> {
        let (config, config_path) = {
            let mut inner = self.inner.lock().expect("server controller poisoned");
            if inner.process.is_some() {
                return Err(PumpkinError::ServerAlreadyRunning);
            }

            inner.status = ServerStatus::Starting;
            inner.last_error = None;
            (inner.config.clone(), inner.config_path.clone())
        };

        let base_directory = config_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf();
        let executable = resolve_path(&base_directory, &config.executable);
        if let Err(error) = ensure_executable(&executable) {
            let mut inner = self.inner.lock().expect("server controller poisoned");
            inner.status = ServerStatus::Failed;
            inner.last_error = Some(error.to_string());
            return Err(error);
        }

        let working_directory = resolve_path(&base_directory, &config.server_directory);
        if !working_directory.exists() {
            let mut inner = self.inner.lock().expect("server controller poisoned");
            inner.status = ServerStatus::Failed;
            inner.last_error = Some(format!(
                "missing working directory: {}",
                working_directory.display()
            ));
            return Err(PumpkinError::MissingWorkingDirectory(working_directory));
        }

        let mut command = Command::new(&executable);
        command
            .current_dir(&working_directory)
            .args(&config.arguments)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = command.spawn().map_err(|error| {
            let mut inner = self.inner.lock().expect("server controller poisoned");
            inner.status = ServerStatus::Failed;
            inner.last_error = Some(error.to_string());
            PumpkinError::StartFailed(error.to_string())
        })?;

        let pid = child.id();
        let stdout = child.stdout.take();
        let stderr = child.stderr.take();
        let stdin = child.stdin.take();

        let child = Arc::new(Mutex::new(child));
        let stdin = match stdin {
            Some(handle) => Arc::new(Mutex::new(handle)),
            None => {
                let mut child = child.lock().expect("server controller poisoned");
                let _ = child.kill();
                let _ = child.wait();
                let mut inner = self.inner.lock().expect("server controller poisoned");
                inner.status = ServerStatus::Failed;
                inner.last_error = Some("stdin pipe unavailable".to_string());
                return Err(PumpkinError::StartFailed(
                    "stdin pipe unavailable".to_string(),
                ));
            }
        };

        {
            let mut inner = self.inner.lock().expect("server controller poisoned");
            inner.status = ServerStatus::Running;
            inner.active_pid = Some(pid);
            inner.process = Some(ServerProcess {
                pid,
                child: child.clone(),
                stdin: stdin.clone(),
            });
            inner.logs.push_back(ServerLogLine {
                stream: ServerStream::Stdout,
                line: format!("process {} started", pid),
            });
            trim_logs(&mut inner.logs);
        }

        if let Some(stdout) = stdout {
            spawn_reader(self.inner.clone(), pid, stdout, ServerStream::Stdout);
        }

        if let Some(stderr) = stderr {
            spawn_reader(self.inner.clone(), pid, stderr, ServerStream::Stderr);
        }

        spawn_watcher(self.inner.clone(), pid, child);
        Ok(())
    }

    pub fn stop(&self) -> Result<()> {
        let process = {
            let mut inner = self.inner.lock().expect("server controller poisoned");
            let Some(process) = inner.process.take() else {
                return Err(PumpkinError::ServerNotRunning);
            };

            inner.status = ServerStatus::Stopping;
            inner.last_error = None;
            process
        };

        let stop_write_result = {
            let mut stdin = process.stdin.lock().expect("server controller poisoned");
            stdin.write_all(b"stop\n").and_then(|_| stdin.flush())
        };

        let controller = self.inner.clone();
        thread::spawn(move || {
            let start = Instant::now();
            loop {
                let finished = {
                    let mut child = process.child.lock().expect("server controller poisoned");
                    matches!(child.try_wait(), Ok(Some(_)))
                };

                if finished {
                    break;
                }

                if start.elapsed() >= STOP_TIMEOUT {
                    let mut child = process.child.lock().expect("server controller poisoned");
                    let _ = child.kill();
                    let _ = child.wait();
                    break;
                }

                thread::sleep(Duration::from_millis(200));
            }

            let mut inner = controller.lock().expect("server controller poisoned");
            if inner.active_pid == Some(process.pid) {
                inner.status = ServerStatus::Stopped;
                inner.active_pid = None;
                inner.last_error = None;
            }
        });

        if let Err(error) = stop_write_result {
            return Err(PumpkinError::StopFailed(error.to_string()));
        }

        Ok(())
    }

    pub fn send_command(&self, command: &str) -> Result<()> {
        let process = {
            let inner = self.inner.lock().expect("server controller poisoned");
            let Some(process) = inner.process.as_ref() else {
                return Err(PumpkinError::ServerNotRunning);
            };

            process.stdin.clone()
        };

        let mut stdin = process.lock().expect("server controller poisoned");
        stdin.write_all(command.as_bytes())?;
        if !command.ends_with('\n') {
            stdin.write_all(b"\n")?;
        }
        stdin.flush()?;
        Ok(())
    }

    pub fn refresh(&self) {
        let mut inner = self.inner.lock().expect("server controller poisoned");
        let Some((pid, child)) = inner
            .process
            .as_ref()
            .map(|process| (process.pid, process.child.clone()))
        else {
            return;
        };

        let running = {
            let mut child = child.lock().expect("server controller poisoned");
            matches!(child.try_wait(), Ok(None))
        };

        if running {
            inner.status = ServerStatus::Running;
            inner.active_pid = Some(pid);
            return;
        }

        inner.status = ServerStatus::Stopped;
        inner.active_pid = None;
        inner.process = None;
    }
}

fn spawn_reader(
    controller: Arc<Mutex<ServerControllerInner>>,
    pid: u32,
    stream: impl std::io::Read + Send + 'static,
    output: ServerStream,
) {
    thread::spawn(move || {
        let reader = BufReader::new(stream);
        for line in reader.lines() {
            let Ok(line) = line else {
                break;
            };

            let mut inner = controller.lock().expect("server controller poisoned");
            if inner.active_pid != Some(pid) {
                break;
            }

            inner.logs.push_back(ServerLogLine {
                stream: output,
                line,
            });
            trim_logs(&mut inner.logs);
        }
    });
}

fn spawn_watcher(
    controller: Arc<Mutex<ServerControllerInner>>,
    pid: u32,
    child: Arc<Mutex<Child>>,
) {
    thread::spawn(move || {
        loop {
            let finished = {
                let mut child = child.lock().expect("server controller poisoned");
                matches!(child.try_wait(), Ok(Some(_)))
            };

            if !finished {
                thread::sleep(Duration::from_millis(250));
                continue;
            }

            let mut inner = controller.lock().expect("server controller poisoned");
            if inner.active_pid == Some(pid) {
                inner.status = ServerStatus::Stopped;
                inner.active_pid = None;
                inner.process = None;
            }
            break;
        }
    });
}

fn trim_logs(logs: &mut VecDeque<ServerLogLine>) {
    while logs.len() > LOG_LIMIT {
        let _ = logs.pop_front();
    }
}
