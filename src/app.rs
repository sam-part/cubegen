use crate::{
    components::{component::Component, timer::TimerComponent},
    event::{Event, EventHandler},
    input::{Action, ActionMap},
};
use color_eyre::eyre::{eyre, Result};
use ratatui::{prelude::CrosstermBackend, Terminal};

pub type Tui = Terminal<CrosstermBackend<std::io::Stdout>>;

pub struct App {
    running: bool,
    event_handler: EventHandler,
    components: Vec<Box<dyn Component>>,
    terminal: Tui,
    action_map: ActionMap,
}

impl Default for App {
    fn default() -> Self {
        Self {
            // TODO: get tickrate from config
            event_handler: EventHandler::new(20.0),
            running: false,
            components: vec![Box::new(TimerComponent::new())],
            terminal: ratatui::init(),
            action_map: ActionMap::default(),
        }
    }
}

impl App {
    pub fn new() -> Self {
        App::default()
    }

    /// Start the application and run the main loop.
    pub async fn run(&mut self) -> Result<()> {
        self.terminal.clear()?;
        self.running = true;

        let result = self.main_loop().await;

        ratatui::restore();

        result
    }

    /// Runs the main loop of the application indefinitely.
    async fn main_loop(&mut self) -> Result<()> {
        while self.running {
            let event = self.event_handler.next().await?;

            self.handle_event(event)?;
        }

        Ok(())
    }

    /// Handles an event, dispatching appropriately to the app's components.
    /// * `event`: The event to be handled
    fn handle_event(&mut self, event: Event) -> Result<()> {
        match event {
            Event::Key(key_event) => {
                if let Some(action) = self.action_map.get_action(key_event) {
                    if *action == Action::Quit {
                        self.running = false;
                        return Ok(());
                    }

                    for component in self.components.iter_mut() {
                        component.handle_action(action)?;
                    }
                }
            }

            Event::Mouse(mouse_event) => {
                for component in self.components.iter_mut() {
                    component.handle_mouse_event(mouse_event, &self.action_map)?;
                }
            }

            Event::Resize(_width, _height) => {}

            Event::Error(error) => return Err(error),

            Event::Tick => {
                for component in self.components.iter_mut() {
                    component.update()?;
                }

                self.draw()?;
            }
        }

        Ok(())
    }

    /// Attempts to draw each component to the terminal.
    fn draw(&mut self) -> Result<()> {
        self.terminal.draw(|frame| {
            for component in self.components.iter_mut() {
                // Attempt to draw component
                if let Err(error) = component.draw(frame, frame.area()) {
                    // If an error is received, send it to be handled in the main loop
                    self.event_handler
                        .send(Event::Error(eyre!("Could not draw component: {}", error)));
                }
            }
        })?;

        Ok(())
    }
}
