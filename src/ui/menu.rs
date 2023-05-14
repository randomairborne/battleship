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

pub fn select_play_mode(stdout: &mut Stdout) -> Result<PlayMode, Error> {
    let (width, height) = crossterm::terminal::size()?;
    let play_mode = pick_mode(stdout, width, height)?;
    queue!(stdout, Clear(crossterm::terminal::ClearType::All))?;
    let mut in_progress = String::with_capacity(512);
    match play_mode {
        PlayMode::Local => return Ok(PlayMode::Local),
        PlayMode::Join(_) => in_progress = "127.0.0.1:9416".to_string(),
        PlayMode::Host(_) => in_progress = "9416".to_string(),
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
            PlayMode::Local => break,
            PlayMode::Join(_sock_addr) => in_progress
                .parse::<SocketAddr>()
                .err()
                .map(|v| format!("Address parse error: {v}")),
            PlayMode::Host(_port) => in_progress
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
    match play_mode {
        PlayMode::Local => {}
        PlayMode::Join(mut v) => v = std::net::SocketAddr::from_str(&in_progress)?,
        PlayMode::Host(mut p) => p = in_progress.parse()?,
    }
    Ok(play_mode)
}

fn pick_mode(stdout: &mut Stdout, term_width: u16, term_height: u16) -> Result<PlayMode, Error> {
    queue!(stdout, Clear(crossterm::terminal::ClearType::All), Hide)?;
    let mut play_mode = PlayMode::Local;
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
    queue!(stdout, Show)?;
    Ok(play_mode)
}
