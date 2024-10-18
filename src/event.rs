use color_eyre::eyre::{eyre, Report, Result};
use crossterm::{
    self,
    event::{Event as CrosstermEvent, EventStream},
};
use futures::{future::FutureExt, StreamExt};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

pub enum Event {
    Key(crossterm::event::KeyEvent),
    Mouse(crossterm::event::MouseEvent),
    Resize(u16, u16),
    Error(Report),
    Tick,
}

pub struct EventHandler {
    tx: UnboundedSender<Event>,
    rx: UnboundedReceiver<Event>,
}

impl EventHandler {
    pub fn new(tick_rate: f64) -> Self {
        let tick_period = std::time::Duration::from_secs_f64(1.0 / tick_rate);
        let mut tick_interval = tokio::time::interval(tick_period);

        let (tx, rx) = unbounded_channel();
        let _tx = tx.clone();

        tokio::spawn(async move {
            let mut reader = EventStream::new();

            loop {
                let next_event = reader.next().fuse();

                tokio::select! {
                    // Send a tick event at regular intervals
                    _ = tick_interval.tick() => {
                        tx.send(Event::Tick).unwrap();
                    },

                    // If a crossterm event is received, handle it accordingly
                    event = next_event => {
                        handle_crossterm_event(event, &tx)
                    }
                }
            }
        });

        EventHandler { tx: _tx, rx }
    }

    pub fn send(&self, event: Event) {
        self.tx.send(event).unwrap();
    }

    pub async fn next(&mut self) -> Result<Event> {
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
    tx: &UnboundedSender<Event>,
) {
    match event {
        // Event received
        Some(Ok(event)) => match event {
            CrosstermEvent::Key(key) => {
                tx.send(Event::Key(key)).unwrap();
            }

            CrosstermEvent::Resize(width, height) => {
                tx.send(Event::Resize(width, height)).unwrap();
            }

            CrosstermEvent::Mouse(mouse) => {
                tx.send(Event::Mouse(mouse)).unwrap();
            }

            _ => {}
        },

        // Error received
        Some(Err(err)) => tx.send(Event::Error(err.into())).unwrap(),

        _ => {}
    }
}
