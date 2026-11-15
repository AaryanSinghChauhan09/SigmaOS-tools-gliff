//! SigmaOS Tools Gliff - Hyprland remote desktop protocol and stream orchestrator.

/// Remote display session state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Disconnected,
    Handshake,
    Streaming,
    Suspended,
}

/// Gliff remote input event.
#[derive(Debug, Clone, PartialEq)]
pub enum InputEvent {
    PointerMotion { x: f64, y: f64 },
    PointerButton { button: u32, state: bool },
    Key { keycode: u32, pressed: bool },
}

/// Remote desktop session.
#[derive(Debug, Clone)]
pub struct GliffSession {
    remote_host: String,
    port: u16,
    state: SessionState,
    fps_limit: u32,
    events_queued: usize,
}

impl GliffSession {
    pub fn new(remote_host: &str, port: u16) -> Self {
        Self {
            remote_host: remote_host.to_string(),
            port,
            state: SessionState::Disconnected,
            fps_limit: 60,
            events_queued: 0,
        }
    }

    pub fn connect(&mut self) -> Result<(), &'static str> {
        if self.remote_host.is_empty() {
            return Err("Empty host");
        }
        self.state = SessionState::Handshake;
        self.state = SessionState::Streaming;
        Ok(())
    }

    pub fn disconnect(&mut self) {
        self.state = SessionState::Disconnected;
        self.events_queued = 0;
    }

    pub fn send_input(&mut self, _event: InputEvent) -> bool {
        if self.state == SessionState::Streaming {
            self.events_queued += 1;
            true
        } else {
            false
        }
    }

    pub fn state(&self) -> SessionState {
        self.state
    }

    pub fn host(&self) -> &str {
        &self.remote_host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn events_processed(&self) -> usize {
        self.events_queued
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_lifecycle_aaa() {
        // Arrange
        let mut session = GliffSession::new("192.168.1.100", 7080);
        assert_eq!(session.state(), SessionState::Disconnected);

        // Act
        session.connect().expect("Connect must succeed");

        // Assert
        assert_eq!(session.state(), SessionState::Streaming);
        assert!(session.send_input(InputEvent::PointerMotion { x: 100.0, y: 200.0 }));
        assert_eq!(session.events_processed(), 1);

        session.disconnect();
        assert_eq!(session.state(), SessionState::Disconnected);
        assert!(!session.send_input(InputEvent::PointerMotion { x: 0.0, y: 0.0 }));
    }
}
