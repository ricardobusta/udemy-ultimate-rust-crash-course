use std::error::Error;
use std::{io, thread};
use std::ptr::null;
use std::sync::mpsc;
use std::time::Duration;
use crossterm::{event, ExecutableCommand, terminal};
use crossterm::cursor::{Hide, Show};
use crossterm::event::KeyCode;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use rusty_audio::Audio;
use z_invaders::frame;
use z_invaders::render;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();

    audio.add("explode", "z_invaders/assets/explode.wav");
    audio.add("lose", "z_invaders/assets/lose.wav");
    audio.add("move", "z_invaders/assets/move.wav");
    audio.add("pew", "z_invaders/assets/pew.wav");
    audio.add("startup", "z_invaders/assets/startup.wav");
    audio.add("win", "z_invaders/assets/win.wav");
    audio.play("startup");

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?; // Go back to the terminal like vim
    stdout.execute(Hide)?; // hide the cursor

    // Render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop{
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    // Game Loop
    'gameloop: loop {
        // Per-frame init
        let curr_frame = frame::new_frame();

        // Input
        while event::poll(Duration::default())? {
            if let crossterm::event::Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        // Draw & render
        let _ = render_tx.send(curr_frame); // silently ignore result. first frames will crash.
        thread::sleep(Duration::from_millis(1)); // prevent thousands of updates
    }

    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}