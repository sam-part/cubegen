use crate::event::Event;
use crate::input::{Action, ActionMap};
use color_eyre::eyre::Result;
use crossterm::event::MouseEvent;
use ratatui::{layout::Rect, Frame};

/// A component is a distinct visual and interactive element of the application.
/// Managed and updated by the main application loop.
pub trait Component {
    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        Ok(())
    }

    fn handle_event(&mut self, event: Event, action_map: &ActionMap) -> Result<()> {
        match event {
            Event::Mouse(mouse_event) => self.handle_mouse_event(mouse_event, action_map),
            Event::Tick => self.update(),

            _ => Ok(()),
        }
    }

    fn handle_action(&mut self, _action: &Action) -> Result<()> {
        Ok(())
    }

    fn handle_mouse_event(&mut self, _event: MouseEvent, _action_map: &ActionMap) -> Result<()> {
        Ok(())
    }

    fn draw(&mut self, _frame: &mut Frame, _area: Rect) -> Result<()> {
        Ok(())
    }

    // fn list_commands(&self) -> Vec<CommandInfo>;
}
