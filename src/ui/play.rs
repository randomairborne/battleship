use std::io::{Stdout, Write};

use crate::board::{Board, Shot};
use crate::cell::Cell;
use crate::error::Error;
use crossterm::{
    cursor::MoveTo,
    event::{KeyCode, KeyModifiers},
    execute, queue,
    style::{Color, Print, PrintStyledContent, Stylize},
    terminal::Clear,
};

use super::{exit, wait_on_player};

pub fn turn(
    stdout: &mut Stdout,
    attacker: &mut Board,
    defender: &mut Board,
    cursor: &mut Cell,
    player: usize,
) -> Result<(), Error> {
    crate::ui::show_pass(stdout, player)?;
    let mut msg = String::with_capacity(128);
    render_screen(stdout, attacker, defender, cursor, player, "")?;
    loop {
        let key = crate::util::next_key()?;
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            exit();
        }
        if key.code == KeyCode::Esc {
            exit();
        }
        match key.code {
            KeyCode::Left => *cursor -= (1, 0),
            KeyCode::Right => *cursor += (1, 0),
            KeyCode::Up => *cursor -= (0, 1),
            KeyCode::Down => *cursor += (0, 1),
            KeyCode::Char(' ') => {
                if let Some(shot) = defender.fire(cursor) {
                    msg = match shot {
                        Shot::Hit(ship) => {
                            let mut verb = "sunk";
                            for cell in ship.occupies() {
                                if !matches!(defender.shot(&cell), Shot::Hit(_ship)) {
                                    verb = "hit";
                                    break;
                                }
                            }
                            format!("You {verb} their {}!", ship.kind())
                        }
                        Shot::Miss => "You missed.".to_string(),
                        Shot::Empty => "Shot is empty!?".to_string(),
                    };
                    break;
                }
                msg = "You already shot there!".to_string();
            }
            _ => {}
        }
        queue!(
            stdout,
            MoveTo(0, 13),
            Clear(crossterm::terminal::ClearType::CurrentLine)
        )?;
        render_screen(stdout, attacker, defender, cursor, player, &msg)?;
    }
    render_screen(stdout, attacker, defender, cursor, player, &msg)?;
    execute!(stdout, MoveTo(0, 0))?;
    *cursor = Cell::new(0, 0);
    wait_on_player()?;
    Ok(())
}

pub fn render_screen(
    stdout: &mut Stdout,
    attacker: &mut Board,
    defender: &mut Board,
    cursor: &mut Cell,
    player: usize,
    message: &str,
) -> Result<(), Error> {
    draw_board(stdout, defender, false, 0)?;
    draw_board(stdout, attacker, true, 30)?;
    queue!(
        stdout,
        MoveTo(0, 0),
        Print(player),
        MoveTo(0, 13),
        Print(message),
        #[allow(clippy::cast_possible_truncation)]
        MoveTo(cursor.x() as u16 * 2 + 2, cursor.y() as u16 + 1)
    )?;
    stdout.flush()?;
    Ok(())
}

const HIT_STR: &str = "><";

fn draw_board(
    stdout: &mut Stdout,
    board: &mut Board,
    show_ships: bool,
    x_offset: u16,
) -> Result<(), Error> {
    for x in 1..11 {
        queue!(stdout, MoveTo(x * 2 + x_offset, 0), Print(x))?;
    }
    for y in 1..11 {
        queue!(
            stdout,
            MoveTo(x_offset, y),
            Print(char::from_u32('A' as u32 + (u32::from(y) - 1)).unwrap_or('X'))
        )?;
    }
    for x in 0..10 {
        for y in 0..10 {
            queue!(stdout, MoveTo((x + 1) * 2 - 1 + x_offset, y + 1))?;
            let cell = Cell::new(x.into(), y.into());
            let bg_color = if show_ships && board.ships.contains_ship(cell) {
                Color::Grey
            } else {
                Color::DarkBlue
            };
            match board.shot(&cell) {
                Shot::Hit(_kind) => {
                    queue!(
                        stdout,
                        PrintStyledContent(HIT_STR.with(Color::DarkRed).on(bg_color))
                    )
                }
                Shot::Miss => {
                    queue!(
                        stdout,
                        PrintStyledContent(HIT_STR.with(Color::White).on(bg_color))
                    )
                }
                Shot::Empty => queue!(stdout, PrintStyledContent("  ".on(bg_color))),
            }?;
        }
    }
    Ok(())
}
