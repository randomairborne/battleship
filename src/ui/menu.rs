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

pub enum PlayMode {
    Local,
    Join(SocketAddr),
    Host(u16),
}

pub enum DatalessPlayMode {
    Local,
    Join,
    Host,
}

pub fn select_play_mode(stdout: &mut Stdout) -> Result<PlayMode, Error> {
    let (width, height) = crossterm::terminal::size()?;
    let play_mode = pick_mode(stdout, width, height)?;
    queue!(stdout, Clear(crossterm::terminal::ClearType::All))?;
    let mut in_progress;
    match play_mode {
        DatalessPlayMode::Local => return Ok(PlayMode::Local),
        DatalessPlayMode::Join=> in_progress = "127.0.0.1:9416".to_string(),
        DatalessPlayMode::Host  => in_progress = "9416".to_string(),
    }
    loop {
        queue!(
            stdout,
            MoveTo(width / 2 - 30, height / 2),
            Clear(crossterm::terminal::ClearType::FromCursorDown),
            PrintStyledContent(
                format!("{in_progress: <60}")
                    .blue()
                    .on_grey()
                    .underline_black()
            ),
        )?;
        let parse_error = match play_mode {
            DatalessPlayMode::Local => break,
            DatalessPlayMode::Join => in_progress
                .parse::<SocketAddr>()
                .err()
                .map(|v| format!("Address parse error: {v}")),
            DatalessPlayMode::Host => in_progress
                .parse::<u16>()
                .err()
                .map(|v| format!("Port parse error: {v}")),
        };
        if let Some(e) = parse_error {
            queue!(
                stdout,
                MoveTo(width / 2 - 30, height / 2 + 1),
                PrintStyledContent(format!("{e: <60}").red().on_grey().underline_dark_red())
            )?;
        }
        queue!(
            stdout,
            MoveTo(
                width / 2 - 30 + in_progress.len().try_into().unwrap_or(0),
                height / 2
            )
        )?;
        stdout.flush()?;
        match crate::util::next_key()?.code {
            KeyCode::Char(ch) => in_progress.push(ch),
            KeyCode::Backspace => {
                in_progress.pop();
            }
            KeyCode::Esc => exit(),
            _ => {}
        };
    }
    let final_mode = match play_mode {
        DatalessPlayMode::Local => PlayMode::Local,
        DatalessPlayMode::Join => PlayMode::Join(std::net::SocketAddr::from_str(&in_progress)?),
        DatalessPlayMode::Host => PlayMode::Host(in_progress.parse()?),
    };
    Ok(final_mode)
}

fn pick_mode(stdout: &mut Stdout, term_width: u16, term_height: u16) -> Result<DatalessPlayMode, Error> {
    queue!(stdout, Clear(crossterm::terminal::ClearType::All), Hide)?;
    let mut play_mode = DatalessPlayMode::Local;
    loop {
        let pass_n_play = if matches!(play_mode, DatalessPlayMode::Local) {
            "   Pass 'n Play    ".on_dark_blue().grey()
        } else {
            "   Pass 'n Play    ".on_grey().dark_blue()
        };
        let join = if matches!(play_mode, DatalessPlayMode::Join) {
            " Join network game ".on_dark_blue().grey()
        } else {
            " Join network game ".on_grey().dark_blue()
        };
        let host = if matches!(play_mode, DatalessPlayMode::Host) {
            " Host network game ".on_dark_blue().grey()
        } else {
            " Host network game ".on_grey().dark_blue()
        };
        queue!(
            stdout,
            MoveTo(term_width / 2 - 10, term_height / 2 - 1),
            PrintStyledContent(pass_n_play),
            MoveTo(term_width / 2 - 10, term_height / 2),
            PrintStyledContent(join),
            MoveTo(term_width / 2 - 10, term_height / 2 + 1),
            PrintStyledContent(host),
        )?;
        stdout.flush()?;
        match crate::util::next_key()?.code {
            KeyCode::Up => {
                play_mode = match play_mode {
                    DatalessPlayMode::Local => DatalessPlayMode::Host,
                    DatalessPlayMode::Join => DatalessPlayMode::Local,
                    DatalessPlayMode::Host => DatalessPlayMode::Join,
                }
            }
            KeyCode::Down => {
                play_mode = match play_mode {
                    DatalessPlayMode::Local => DatalessPlayMode::Join,
                    DatalessPlayMode::Join => DatalessPlayMode::Host,
                    DatalessPlayMode::Host => DatalessPlayMode::Local,
                }
            }
            KeyCode::Char(' ') => break,
            KeyCode::Esc => exit(),
            _ => {}
        };
    }
    queue!(stdout, Show)?;
    Ok(play_mode)
}
