use std::process::{Child, Command, Stdio};

use crate::error::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlayerState {
    #[default]
    Idle,
    Playing,
    Error,
}

#[derive(Debug, Default)]
pub struct Player {
    child: Option<Child>,
    state: PlayerState,
    last_error: Option<String>,
    current_url: Option<String>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            child: None,
            state: PlayerState::Idle,
            last_error: None,
            current_url: None,
        }
    }

    pub fn play(&mut self, url: &str) -> Result<()> {
        self.stop();

        let child = Command::new("mpv")
            .args(["--no-video", "--quiet", "--no-terminal", url])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|err| Error::new(format!("failed to start mpv: {err}")))?;

        self.child = Some(child);
        self.current_url = Some(url.to_string());
        self.state = PlayerState::Playing;
        self.last_error = None;
        Ok(())
    }

    pub fn stop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
        self.state = PlayerState::Idle;
        self.last_error = None;
        self.current_url = None;
    }

    pub fn update(&mut self) {
        if let Some(child) = &mut self.child {
            match child.try_wait() {
                Ok(Some(status)) => {
                    self.child = None;
                    self.current_url = None;
                    self.state = PlayerState::Error;
                    self.last_error = Some(if status.success() {
                        "mpv stopped".to_string()
                    } else {
                        format!("mpv exited with {status}")
                    });
                }
                Ok(None) => {}
                Err(err) => {
                    self.child = None;
                    self.current_url = None;
                    self.state = PlayerState::Error;
                    self.last_error = Some(format!("failed to query mpv: {err}"));
                }
            }
        }
    }

    pub fn state(&self) -> PlayerState {
        self.state
    }

    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }

    pub fn current_url(&self) -> Option<&str> {
        self.current_url.as_deref()
    }
}
