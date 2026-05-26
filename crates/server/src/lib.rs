use std::sync::Arc;
use tokio::sync::broadcast;
use domain::Article;

/// Core events emitted by the system
#[derive(Debug, Clone)]
pub enum SystemEvent {
    ArticleCreated(Article),
    ArticleUpdated(Article),
}

/// The EventBus manages publish/subscribe for system events.
/// We use tokio broadcast channels as the initial foundation before NATS.
pub struct EventBus {
    sender: broadcast::Sender<SystemEvent>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<SystemEvent> {
        self.sender.subscribe()
    }

    pub fn publish(&self, event: SystemEvent) -> Result<usize, broadcast::error::SendError<SystemEvent>> {
        self.sender.send(event)
    }
}

/// Shared application state
pub struct AppState {
    pub event_bus: Arc<EventBus>,
    // TODO: Include storage/database pool here
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        Self {
            event_bus: Arc::new(EventBus::new(1024)),
        }
    }
}
