use crate::app::AppContext;
use crate::{clock::Clock, components::component::Component, input::Action};
use color_eyre::eyre::Result;
use ratatui::{layout::Rect, widgets::Paragraph, Frame};

#[derive(Default)]
pub struct TimerComponent {
    clock: Clock,
    release_clock: Clock,
    last_solve_time: f64,
}

impl TimerComponent {
    pub fn new() -> Self {
        TimerComponent::default()
    }
}

impl Component for TimerComponent {
    fn handle_action(&mut self, action: &Action, context: &AppContext) -> Result<()> {
        match action {
            // Due to terminal limitations this will only work on supported terminal emulators (eg. kitty).
            // Disabled by default, can be enabled in the config.
            Action::TimerStartRelease if context.config.timer.use_key_release => {
                if self.release_clock.elapsed().as_secs_f64() >= context.config.timer.freeze_time
                    && !self.clock.is_running()
                {
                    self.clock.start();
                    self.release_clock.stop();
                }
            }
            Action::TimerToggle => {
                if !self.clock.is_running() {
                    if context.config.timer.use_key_release {
                        self.release_clock.start();
                    } else {
                        self.clock.start();
                    }
                } else if self.clock.is_running() && !self.clock.elapsed().is_zero() {
                    self.last_solve_time = self.clock.stop().as_secs_f64();
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect, context: &AppContext) -> Result<()> {
        let time = {
            if self.clock.is_running() {
                self.clock.elapsed().as_secs_f64()
            } else {
                self.last_solve_time
            }
        };

        let decimal_digits = context.config.timer.display_decimal_points;
        let time_str = format!("{:.*}", decimal_digits, time);

        frame.render_widget(Paragraph::new(time_str), area);

        Ok(())
    }
}
