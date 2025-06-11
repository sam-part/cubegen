use crate::{
    components::{component::Component, timer::TimerComponent},
    config::AppConfig,
    event::{Event, EventHandler},
    input::{Action, ActionMap},
};
use color_eyre::eyre::{eyre, Result};
use crossterm::event::KeyboardEnhancementFlags;
use crossterm::{
    event::{PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
    execute,
};
use ratatui::layout::Margin;
use ratatui::symbols::border;
use ratatui::text::Line;
use ratatui::widgets::Block;
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::io;

pub type Tui = Terminal<CrosstermBackend<std::io::Stdout>>;

/// Read-only application context that components have access to
pub struct AppContext {
    pub(crate) config: AppConfig,
    action_map: ActionMap,
}
pub struct App {
    running: bool,
    context: AppContext,
    terminal: Tui,
    components: Vec<Box<dyn Component>>,
    event_handler: EventHandler,
}
impl App {
    pub fn new() -> Result<Self> {
        let config = AppConfig::load()?;
        let tickrate = config.tickrate;

        let context = AppContext {
            config,
            action_map: ActionMap::default(),
        };

        let app = Self {
            running: false,
            context,
            terminal: ratatui::init(),
            components: vec![Box::new(TimerComponent::new())],
            event_handler: EventHandler::new(tickrate),
        };

        Ok(app)
    }

    /// Start the application and run the main loop.
    pub async fn run(&mut self) -> Result<()> {
        self.terminal.clear()?;
        self.running = true;

        let mut stdout = io::stdout();

        if self.context.config.timer.use_key_release {
            execute!(
                stdout,
                PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES)
            )?;
        }

        let result = self.main_loop().await;

        ratatui::restore();

        if self.context.config.timer.use_key_release {
            execute!(stdout, PopKeyboardEnhancementFlags)?;
        }

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
                if let Some(action) = self.context.action_map.get_action(key_event) {
                    if *action == Action::Quit {
                        self.running = false;
                        return Ok(());
                    }

                    for component in self.components.iter_mut() {
                        component.handle_action(action, &self.context)?;
                    }
                }
            }

            Event::Mouse(mouse_event) => {
                for component in self.components.iter_mut() {
                    component.handle_mouse_event(mouse_event, &self.context)?;
                }
            }

            Event::Resize(_width, _height) => {}

            Event::Error(error) => return Err(error),

            Event::Tick => {
                for component in self.components.iter_mut() {
                    component.update(&self.context)?;
                }

                self.draw()?;
            }
        }

        Ok(())
    }

    /// Attempts to draw each component to the terminal.
    fn draw(&mut self) -> Result<()> {
        self.terminal.draw(|frame| {
            let title = Line::from(" cubegen ").centered();
            let title_bottom = Line::from(" Press ? for help ").right_aligned();

            let block = Block::bordered()
                .title(title)
                .title_bottom(title_bottom)
                .border_set(border::THICK);

            frame.render_widget(block, frame.area());

            let area = frame.area().inner(Margin::new(1, 1));

            for component in self.components.iter_mut() {
                // Attempt to draw component
                if let Err(error) = component.draw(frame, area, &self.context) {
                    // If an error is received, send it to be handled in the main loop
                    self.event_handler
                        .send(Event::Error(eyre!("Could not draw component: {}", error)));
                }
            }
        })?;

        Ok(())
    }
}
