use std::{
    io::{Stdout, Write},
    net::SocketAddr,
    str::FromStr,
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::KeyCode,
    queue,
    style::{PrintStyledContent, Stylize},
    terminal::Clear,
};

use crate::Error;

use super::exit;

pub fn select_play_mode(stdout: &mut Stdout) -> Result<PlayMode, Error> {
    let (width, height) = crossterm::terminal::size()?;
    let mut play_mode = PlayMode::Local;
    queue!(stdout, Clear(crossterm::terminal::ClearType::All),)?;
    loop {
        let pass_n_play = if matches!(play_mode, PlayMode::Local) {
            "   Pass 'n Play    ".on_dark_blue().grey()
        } else {
            "   Pass 'n Play    ".on_grey().dark_blue()
        };
        let join = if matches!(play_mode, PlayMode::Join(_)) {
            " Join network game ".on_dark_blue().grey()
        } else {
            " Join network game ".on_grey().dark_blue()
        };
        let host = if matches!(play_mode, PlayMode::Host(_)) {
            " Host network game ".on_dark_blue().grey()
        } else {
            " Host network game ".on_grey().dark_blue()
        };
        queue!(
            stdout,
            MoveTo(width / 2 - 10, height / 2 - 1),
            PrintStyledContent(pass_n_play),
            MoveTo(width / 2 - 10, height / 2),
            PrintStyledContent(join),
            MoveTo(width / 2 - 10, height / 2 + 1),
            PrintStyledContent(host),
            Hide
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
            KeyCode::Esc => exit(),
            _ => {}
        };
    }
    execute!(stdout, Show)?;
    Ok(play_mode)
}

pub enum PlayMode {
    Local,
    Join(SocketAddr),
    Host(u16),
}
