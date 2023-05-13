pub mod menu;
pub mod play;
pub mod setup;

use crate::error::Error;

use crossterm::{
    cursor::{MoveTo, Show},
    event::{KeyCode, KeyModifiers},
    queue,
    style::Print,
    terminal::Clear,
};
use std::io::{Stdout, Write};

pub fn show_pass(stdout: &mut Stdout, player: usize) -> Result<(), Error> {
    queue!(
        stdout,
        Clear(crossterm::terminal::ClearType::All),
        MoveTo(2, 2),
        Print("Pass the game to player "),
        Print(player)
    )?;
    stdout.flush()?;
    wait_on_player()?;
    Ok(())
}

pub fn wait_on_player() -> Result<(), Error> {
    loop {
        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            if key.code == KeyCode::Enter || key.code == KeyCode::Char(' ') {
                break;
            }
            if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                exit();
            }
            if key.code == KeyCode::Esc {
                exit();
            }
        }
    }
    Ok(())
}

pub fn exit() -> ! {
    crossterm::execute!(
        std::io::stdout(),
        Clear(crossterm::terminal::ClearType::All),
        MoveTo(0, 0),
        Show,
    )
    .ok();
    crossterm::terminal::disable_raw_mode()
        .expect("Failed to disable raw mode - terminal may be corrupted");
    println!("Thanks for playing!");
    std::process::exit(0)
}

pub fn clear_msgs(out: &mut Stdout) -> Result<(), Error> {
    queue!(
        out,
        MoveTo(0, 11),
        Clear(crossterm::terminal::ClearType::FromCursorDown)
    )?;
    Ok(())
}
