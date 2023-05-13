use crate::board::Board;
use crate::cell::Cell;
use crate::error::Error;
use crate::ship::{ShipRotation, ShipSetBuilder, ShipState, ShipType};
use crossterm::{
    cursor::MoveTo,
    event::{KeyCode, KeyModifiers},
    queue,
    style::{Print, PrintStyledContent, Stylize},
    terminal::Clear,
};
use std::io::{Stdout, Write};

pub fn do_place(
    stdout: &mut Stdout,
    cursor: &mut Cell,
    player: usize,
    action: &str,
) -> Result<Board, Error> {
    let mut ships = ShipSetBuilder::new();
    let mut ship_rot = ShipRotation::Down;
    let mut ship = ShipType::AircraftCarrier;
    let mut last_action_was_place = false;
    let mut message = action.to_string();
    ships.carrier(ShipState::new(*cursor, ship_rot, ShipType::AircraftCarrier));
    draw_ship_picker(stdout, &ships, player, &message, cursor)?;
    loop {
        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                super::exit();
            }
            if key.code == KeyCode::Esc {
                super::exit();
            }
            match key.code {
                KeyCode::Left | KeyCode::Char('A' | 'a') => *cursor -= (1, 0),
                KeyCode::Right | KeyCode::Char('D' | 'd') => *cursor += (1, 0),
                KeyCode::Up | KeyCode::Char('W' | 'w') => *cursor -= (0, 1),
                KeyCode::Down | KeyCode::Char('S' | 's') => *cursor += (0, 1),
                KeyCode::Char('e' | 'E' | '?' | '/') => ship_rot.next(),
                KeyCode::Char('q' | 'Q' | '>' | '.') => ship_rot.prev(),
                KeyCode::Char(' ') | KeyCode::Enter => {
                    if ships.is_valid() && ship.next() {
                        if let Some(finished) = ships.build() {
                            *cursor = Cell::new(0, 0);
                            return Ok(Board::new(finished));
                        }
                        message = "Board is valid but is invalid!?".to_string();
                    }
                    last_action_was_place = true;
                }
                _ => {}
            }
        }
        match ship {
            ShipType::AircraftCarrier => {
                ships.carrier(ShipState::new(*cursor, ship_rot, ShipType::AircraftCarrier));
            }
            ShipType::Battleship => {
                ships.battleship(ShipState::new(*cursor, ship_rot, ShipType::Battleship));
            }
            ShipType::Destroyer => {
                ships.destroyer(ShipState::new(*cursor, ship_rot, ShipType::Destroyer));
            }
            ShipType::Submarine => {
                ships.submarine(ShipState::new(*cursor, ship_rot, ShipType::Submarine));
            }
            ShipType::PatrolBoat => {
                ships.patrol(ShipState::new(*cursor, ship_rot, ShipType::PatrolBoat));
            }
        }
        if !ships.is_valid() && !last_action_was_place {
            message = "Invalid board layout".to_string();
        }
        draw_ship_picker(stdout, &ships, player, &message, cursor)?;
        message.clear();
        last_action_was_place = false;
    }
}

fn draw_ship_picker(
    stdout: &mut Stdout,
    ships: &ShipSetBuilder,
    player: usize,
    message: &str,
    cursor: &Cell,
) -> Result<(), Error> {
    queue!(stdout, Clear(crossterm::terminal::ClearType::All))?;
    for x in 1..11 {
        queue!(stdout, MoveTo(x * 2, 0), Print(x))?;
    }
    for y in 1..11 {
        queue!(
            stdout,
            MoveTo(0, y),
            Print(char::from_u32('A' as u32 + (u32::from(y) - 1)).unwrap_or('X'))
        )?;
    }
    for x in 0..10 {
        for y in 0..10 {
            let cell = Cell::new(x.into(), y.into());
            let on_color = if ships.contains_ship(cell) {
                Stylize::on_grey
            } else {
                Stylize::on_blue
            };
            queue!(
                stdout,
                MoveTo((x + 1) * 2 - 1, y + 1),
                PrintStyledContent(on_color("  "))
            )?;
        }
    }
    queue!(
        stdout,
        MoveTo(0, 13),
        Print(message),
        MoveTo(0, 0),
        Print(player),
        #[allow(clippy::cast_possible_truncation)]
        MoveTo(cursor.x() as u16 * 2 + 2, cursor.y() as u16 + 1)
    )?;
    stdout.flush()?;
    Ok(())
}
