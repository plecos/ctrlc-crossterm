/// This Rust program demonstrates how to handle Ctrl-C (Control + C) key press events
/// using the `crossterm` crate for terminal manipulation and event handling. The program
/// runs a loop that waits for the Ctrl-C key press and exits gracefully when it is detected.
///
/// The main components of the program are:
///
/// - `running`: A thread-safe boolean flag wrapped in an `Arc<AtomicBool>` to indicate
///   whether the application is running. This allows shared ownership across multiple
///   threads and enables atomic operations on the boolean value.
/// - A spawned thread that continuously polls for keyboard events using `crossterm::event::poll`.
///   When a Ctrl-C key press is detected, it sets the `running` flag to `false`.
/// - The main thread enables raw mode for the terminal using `crossterm::terminal::enable_raw_mode`
///   and enters a loop that waits until the `running` flag is set to `false`.
///
/// The program prints "Waiting for Ctrl-C..." to the terminal and exits with "Got it! Exiting..."
/// when the Ctrl-C key press is detected.
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crossterm::{cursor, queue, style};
use crossterm::{execute, terminal, style::Print};
use crossterm::event::{self, Event, KeyCode};

fn main() -> Result<(), anyhow::Error> {
    // A thread-safe boolean flag to indicate whether the application is running.
    // This is wrapped in an `Arc` to allow shared ownership across multiple threads,
    // and an `AtomicBool` to enable atomic operations on the boolean value.
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    std::thread::spawn(move || -> Result<(), anyhow::Error> {
        loop {
            // using a 100 ms timeout to be cpu friendly
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    if key_event.code == KeyCode::Char('c') && key_event.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
                        // signal by settting our AtomicBool to false
                        r.store(false, Ordering::SeqCst);
                    }
                }
            }
        }
    });

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

