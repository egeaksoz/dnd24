pub mod app;
pub mod button;
pub mod effect_registry;
pub mod gruvbox;

use std::{error::Error, io, time::Instant};

use app::App;
use color_eyre::Result;
use crossterm::event::{self};
use ratatui::backend::CrosstermBackend;
use tachyonfx::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(
        io::stdout(),
        crossterm::terminal::EnterAlternateScreen,
        crossterm::event::EnableMouseCapture,
    )?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = ratatui::Terminal::new(backend)?;

    // Create app
    let mut app = App::new();
    let mut last_tick = Instant::now();

    // Main loop
    while app.is_running {
        let timeout = std::time::Duration::from_millis(16); // ~60 FPS
        let now = Instant::now();
        let elapsed = now.duration_since(last_tick);
        last_tick = now;

        // Handle events
        if event::poll(timeout)? {
            let event = event::read()?;
            app.handle_event(event);
        }

        // Render
        terminal.draw(|frame| {
            // Render UI
            app.render(frame);

            // Process effects
            let tachyon_duration = Duration::from_millis(elapsed.as_millis() as u32);
            let frame_area = frame.area();
            app.registry
                .process_effects(tachyon_duration, frame.buffer_mut(), frame_area);
        })?;
    }

    // Cleanup
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        io::stdout(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture,
    )?;

    Ok(())
}
