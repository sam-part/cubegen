use color_eyre::eyre::{eyre, Result};
use crossterm::{
    self,
    event::{Event as CrosstermEvent, EventStream},
};
use futures::{future::FutureExt, StreamExt};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

pub enum Message {
    Key(crossterm::event::KeyEvent),
    Mouse(crossterm::event::MouseEvent),
    Resize(u16, u16),
    Error(std::io::Error),
    Tick,
}

pub struct EventHandler {
    rx: UnboundedReceiver<Message>,
}

impl EventHandler {
    pub fn new(tick_rate: f64) -> Self {
        let tick_period = std::time::Duration::from_secs_f64(1.0 / tick_rate);
        let mut tick_interval = tokio::time::interval(tick_period);

        let (tx, rx) = unbounded_channel();

        tokio::spawn(async move {
            let mut reader = EventStream::new();

            loop {
                let next_event = reader.next().fuse();

                tokio::select! {
                    // Send a tick message at regular intervals
                    _ = tick_interval.tick() => {
                        tx.send(Message::Tick).unwrap();
                    },

                    // If a crossterm event is received, handle it accordingly
                    event = next_event => {
                        handle_crossterm_event(event, &tx)
                    }
                }
            }
        });

        EventHandler { rx }
    }

    pub async fn next(&mut self) -> Result<Message> {
        self.rx
            .recv()
            .await
            .ok_or_else(|| eyre!("Unable to get next event"))
    }
}

/// Helper function to handle a crossterm event
/// * `event`: The optional event returned by EventStream::next()
/// * `tx`: The event handler's send interface
fn handle_crossterm_event(
    event: Option<Result<CrosstermEvent, std::io::Error>>,
    tx: &UnboundedSender<Message>,
) {
    match event {
        // Event received
        Some(Ok(event)) => match event {
            CrosstermEvent::Key(key) => {
                tx.send(Message::Key(key)).unwrap();
            }

            CrosstermEvent::Resize(width, height) => {
                tx.send(Message::Resize(width, height)).unwrap();
            }

            _ => {}
        },

        // Error received
        Some(Err(err)) => tx.send(Message::Error(err)).unwrap(),

        _ => {}
    }
}
