use crate::app::AppContext;
use crate::event::Event;
use crate::input::Action;
use color_eyre::eyre::Result;
use crossterm::event::MouseEvent;
use ratatui::{layout::Rect, Frame};

/// A component is a distinct visual and interactive element of the application.
/// Managed and updated by the main application loop.
pub trait Component {
    fn init(&mut self, _context: &AppContext) -> Result<()> {
        Ok(())
    }

    fn update(&mut self, _context: &AppContext) -> Result<()> {
        Ok(())
    }

    fn handle_event(&mut self, event: Event, context: &AppContext) -> Result<()> {
        match event {
            Event::Mouse(mouse_event) => self.handle_mouse_event(mouse_event, context),
            Event::Tick => self.update(context),

            _ => Ok(()),
        }
    }

    fn handle_action(&mut self, _action: &Action, _context: &AppContext) -> Result<()> {
        Ok(())
    }

    fn handle_mouse_event(&mut self, _event: MouseEvent, _context: &AppContext) -> Result<()> {
        Ok(())
    }

    fn draw(&mut self, _frame: &mut Frame, _area: Rect, _context: &AppContext) -> Result<()> {
        Ok(())
    }

    // fn list_commands(&self) -> Vec<CommandInfo>;
}
