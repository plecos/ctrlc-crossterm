/// This Rust program demonstrates how to Ctrl-C (Control + C) key press events
/// will not be detected using the crate ctrlc (or other similar crates) once the
/// crossterm raw mode is enabled.
///
/// This program will run forever until the process is killed.  Ctrl-C will never stop it.
///
/// The program prints "Waiting for Ctrl-C..." to the terminal and exits with "Got it! Exiting..."
/// when the Ctrl-C key press is detected.
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crossterm::{cursor, queue, style};
use crossterm::{execute, terminal, style::Print};
fn main() -> Result<(), anyhow::Error> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        // signal!
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    // once this is enabled, the ctrlc::set_handler will never fire
    terminal::enable_raw_mode()?;

    let _ = queue!(
        std::io::stdout(),
        style::ResetColor,
        terminal::Clear(terminal::ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0, 0)
    );

    
    execute!(std::io::stdout(), Print("Waiting for Ctrl-C..."), cursor::MoveToNextLine(1))?;
    // will run forever until running is set to false
    let mut counter = 1;
    while running.load(Ordering::SeqCst) {
        execute!(std::io::stdout(), Print(format!("{} Wait for it...",counter)),cursor::MoveLeft(99))?;
        std::thread::sleep(std::time::Duration::from_secs(1));
        counter += 1;
    }
    execute!(std::io::stdout(), Print("Got it! Exiting..."),cursor::MoveToNextLine(1))?;

    terminal::disable_raw_mode()?;
    Ok(())
}

