use std::{
    io::{Stdout, Write},
    net::SocketAddr,
    str::FromStr,
};

use crossterm::{
    cursor::MoveTo,
    event::KeyCode,
    queue,
    style::{Print, PrintStyledContent, Stylize},
    terminal::Clear,
};

use crate::Error;

pub fn select_play_mode(stdout: &mut Stdout) -> Result<PlayMode, Error> {
    let (width, height) = crossterm::terminal::size()?;
    let mut play_mode = PlayMode::Local;
    loop {
        queue!(
            stdout,
            Clear(crossterm::terminal::ClearType::All),
            MoveTo(width / 2 - 10, height / 2 - 1),
            PrintStyledContent("Pass 'n Play     ".on_dark_blue().grey()),
            MoveTo(width / 2 - 10, height / 2),
            PrintStyledContent("Join network game".on_grey().dark_blue()),
            MoveTo(width / 2 - 10, height / 2 + 1),
            PrintStyledContent("Host network game".on_grey().dark_blue())
        )?;
        stdout.flush()?;
        match crate::util::next_key()?.code {
            KeyCode::Up => {
                play_mode = match play_mode {
                    PlayMode::Local => PlayMode::Host(0),
                    PlayMode::Join(_) => PlayMode::Local,
                    PlayMode::Host(_) => PlayMode::Join(SocketAddr::from_str("0.0.0.0:0").unwrap()),
                }
            }
            KeyCode::Down => {
                play_mode = match play_mode {
                    PlayMode::Local => PlayMode::Join(SocketAddr::from_str("0.0.0.0:0").unwrap()),
                    PlayMode::Join(_) => PlayMode::Host(0),
                    PlayMode::Host(_) => PlayMode::Local,
                }
            }
            KeyCode::Char(' ') => break,
            _ => {}
        };
    }
    Ok(play_mode)
}

pub enum PlayMode {
    Local,
    Join(SocketAddr),
    Host(u16),
}
