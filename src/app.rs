use crate::components::component::Component;
use crate::{Event, EventHandler};
use color_eyre::eyre::Result;

struct App {
    running: bool,
    event_handler: EventHandler,
    components: Vec<Box<dyn Component>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            // TODO: get tickrate from config
            event_handler: EventHandler::new(10.0),
            running: false,
            components: Vec::default(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.running = true;

        // while self.running {}

        Ok(())
    }
}
