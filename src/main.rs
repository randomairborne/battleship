#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::module_name_repetitions)]
mod board;
mod cell;
mod error;
mod ship;

use std::io::{Stdout, Write};

use board::{Board, Shot};
use cell::Cell;

use crossterm::{
    cursor::MoveTo,
    event::{KeyCode, KeyModifiers},
    execute, queue,
    style::{Color, Print, PrintStyledContent, Stylize},
    terminal::Clear,
};
use ship::{ShipRotation, ShipSetBuilder, ShipState, ShipType};

pub use error::Error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    crossterm::terminal::enable_raw_mode().expect("Failed to enable raw mode on terminal");
    std::panic::set_hook(Box::new(|info| {
        crossterm::terminal::disable_raw_mode().ok();
        eprintln!("{info}");
    }));
    let mut stdout = std::io::stdout();
    let mut cursor = Cell::new(0, 0);
    let mut p1 = do_place(&mut stdout, &mut cursor, "P1", "Player 1: Place your ships")?;
    show_pass(&mut stdout)?;
    let mut p2 = do_place(&mut stdout, &mut cursor, "P2", "Player 2: Place your ships")?;
    let winner;
    loop {
        turn(&mut stdout, &mut p1, &mut p2, &mut cursor, "p1")?;
        if p2.ships.lost() {
            winner = "Player 1 wins!".to_string();
            break;
        }
        turn(&mut stdout, &mut p2, &mut p1, &mut cursor, "p2")?;
        if p2.ships.lost() {
            winner = "Player 2 wins!".to_string();
            break;
        }
    }
    crossterm::terminal::disable_raw_mode().ok();
    // todo: fancy end screen
    println!("{winner}");
    Ok(())
}

fn turn(
    stdout: &mut Stdout,
    attacker: &mut Board,
    defender: &mut Board,
    cursor: &mut Cell,
    player: &str,
) -> Result<(), Error> {
    show_pass(stdout)?;
    let mut msg = String::with_capacity(128);
    render_screen(stdout, attacker, defender, cursor, player, "")?;
    loop {
        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
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
                            Shot::Hit(ship) => format!("You hit their {ship}!"),
                            Shot::Miss => "You missed".to_string(),
                            Shot::Empty => "Shot is empty!?".to_string(),
                        };
                        break;
                    }
                    msg = "You already shot there!".to_string();
                }
                _ => {}
            }
            render_screen(stdout, attacker, defender, cursor, player, &msg)?;
        }
    }
    render_screen(stdout, attacker, defender, cursor, player, &msg)?;
    execute!(stdout, MoveTo(0, 0))?;
    *cursor = Cell::new(0, 0);
    wait_on_player()?;
    Ok(())
}

fn show_pass(stdout: &mut Stdout) -> Result<(), Error> {
    queue!(
        stdout,
        Clear(crossterm::terminal::ClearType::All),
        MoveTo(2, 2),
        Print("Pass the game to the other player"),
    )?;
    stdout.flush()?;
    wait_on_player()?;
    Ok(())
}

fn wait_on_player() -> Result<(), Error> {
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

fn do_place(
    stdout: &mut Stdout,
    cursor: &mut Cell,
    player: &str,
    action: &str,
) -> Result<Board, Error> {
    let mut ships = ShipSetBuilder::new();
    let mut ship_rot = ShipRotation::Down;
    let mut ship = ShipType::AircraftCarrier;
    let mut last_action_was_place = false;
    let mut message = action.to_string();
    ships.carrier(ShipState::new(
        *cursor,
        ship_rot,
        false,
        ShipType::AircraftCarrier,
    ));
    draw_ship_picker(stdout, &ships, player, &message, cursor)?;
    loop {
        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
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
                KeyCode::Char('A' | 'a') => ship_rot = ShipRotation::Left,
                KeyCode::Char('D' | 'd') => ship_rot = ShipRotation::Right,
                KeyCode::Char('W' | 'w') => ship_rot = ShipRotation::Up,
                KeyCode::Char('S' | 's') => ship_rot = ShipRotation::Down,
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
            ShipType::AircraftCarrier => ships.carrier(ShipState::new(
                *cursor,
                ship_rot,
                false,
                ShipType::AircraftCarrier,
            )),
            ShipType::Battleship => ships.battleship(ShipState::new(
                *cursor,
                ship_rot,
                false,
                ShipType::Battleship,
            )),
            ShipType::Destroyer => ships.destroyer(ShipState::new(
                *cursor,
                ship_rot,
                false,
                ShipType::Destroyer,
            )),
            ShipType::Submarine => ships.submarine(ShipState::new(
                *cursor,
                ship_rot,
                false,
                ShipType::Submarine,
            )),
            ShipType::PatrolBoat => ships.patrol(ShipState::new(
                *cursor,
                ship_rot,
                false,
                ShipType::PatrolBoat,
            )),
        }
        if !ships.is_valid() && !last_action_was_place {
            message = "Invalid board layout".to_string();
        }
        draw_ship_picker(stdout, &ships, player, &message, cursor)?;
        message.clear();
        last_action_was_place = false;
    }
}

fn render_screen(
    stdout: &mut Stdout,
    attacker: &mut Board,
    defender: &mut Board,
    cursor: &mut Cell,
    player: &str,
    message: &str,
) -> Result<(), Error> {
    queue!(stdout, Clear(crossterm::terminal::ClearType::All))?;
    draw_board(stdout, defender, false, 0)?;
    draw_board(stdout, attacker, true, 40)?;
    queue!(
        stdout,
        MoveTo(0, 0),
        Print(player),
        MoveTo(0, 13),
        Print(message),
        #[allow(clippy::cast_possible_truncation)]
        MoveTo(cursor.x() as u16 * 3 + 3, cursor.y() as u16 + 1)
    )?;
    stdout.flush()?;
    Ok(())
}

const HIT_STR: &str = " X ";

fn draw_board(
    stdout: &mut Stdout,
    board: &mut Board,
    show_ships: bool,
    x_offset: u16,
) -> Result<(), Error> {
    for x in 1..11 {
        queue!(stdout, MoveTo(x * 3 + x_offset, 0), Print(x))?;
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
            queue!(stdout, MoveTo((x + 1) * 3 - 1 + x_offset, y + 1))?;
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
                Shot::Empty => queue!(stdout, PrintStyledContent("   ".on(bg_color))),
            }?;
        }
    }
    Ok(())
}

fn draw_ship_picker(
    stdout: &mut Stdout,
    ships: &ShipSetBuilder,
    player: &str,
    message: &str,
    cursor: &Cell,
) -> Result<(), Error> {
    queue!(stdout, Clear(crossterm::terminal::ClearType::All))?;
    for x in 1..11 {
        queue!(stdout, MoveTo(x * 3, 0), Print(x))?;
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
                MoveTo((x + 1) * 3 - 1, y + 1),
                PrintStyledContent(on_color("   "))
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
        MoveTo(cursor.x() as u16 * 3 + 3, cursor.y() as u16 + 1)
    )?;
    stdout.flush()?;
    Ok(())
}

fn exit() -> ! {
    crossterm::execute!(
        std::io::stdout(),
        Clear(crossterm::terminal::ClearType::All),
        MoveTo(0, 0),
    )
    .ok();
    crossterm::terminal::disable_raw_mode()
        .expect("Failed to disable raw mode - terminal may be corrupted");
    println!("Thanks for playing!");
    std::process::exit(0)
}
