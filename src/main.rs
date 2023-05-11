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
    style::{Print, PrintStyledContent, Stylize},
    terminal::Clear,
};
use ship::{ShipRotation, ShipState, ShipType};

pub use error::Error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    crossterm::terminal::enable_raw_mode().expect("Failed to enable raw mode on terminal");
    std::panic::set_hook(Box::new(|info| {
        crossterm::terminal::disable_raw_mode().ok();
        eprintln!("{info}");
    }));
    let mut stdout = std::io::stdout();
    let mut cursor = Cell::new(0, 0);
    let mut p1 = do_place(&mut stdout, &mut cursor, "Player 1: Place your ships")?;
    show_pass(&mut stdout)?;
    let mut p2 = do_place(&mut stdout, &mut cursor, "Player 2: Place your ships")?;
    loop {
        turn(&mut stdout, &mut p2, &mut cursor)?;
        turn(&mut stdout, &mut p1, &mut cursor)?;
    }
    crossterm::terminal::disable_raw_mode().ok();
    Ok(())
}

fn turn(stdout: &mut Stdout, board: &mut Board, cursor: &mut Cell) -> Result<(), Error> {
    show_pass(stdout)?;
    let mut msg = String::with_capacity(128);
    render_board(stdout, board, cursor, false, "")?;
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
                    if let Some(shot) = board.fire(cursor) {
                        msg = match shot {
                            Shot::Hit => "You hit!",
                            Shot::Miss => "You missed",
                            Shot::Empty => "Shot is empty!?",
                        }
                        .to_string();
                    }
                    break;
                }
                _ => {}
            }
            render_board(stdout, board, cursor, false, "")?;
        }
    }
    render_board(stdout, board, cursor, false, &msg)?;
    execute!(stdout, MoveTo(0, 0))?;
    std::thread::sleep(std::time::Duration::from_secs(3));
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

fn do_place(stdout: &mut Stdout, cursor: &mut Cell, action: &str) -> Result<Board, Error> {
    let mut board = Board::new();
    let mut ship_rot = ShipRotation::Down;
    let mut ship = ShipType::AircraftCarrier;
    let mut last_action_was_place = false;
    let mut message = action.to_string();
    board.ships.carrier(ShipState::new(
        *cursor,
        ship_rot,
        false,
        ShipType::AircraftCarrier,
    ));
    render_board(stdout, &mut board, cursor, true, &message)?;
    loop {
        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                exit();
            }
            if key.code == KeyCode::Esc {
                exit();
            }
            match key.code {
                _ => {}
            }
            match key.code {
                KeyCode::Left => *cursor -= (1, 0),
                KeyCode::Right => *cursor += (1, 0),
                KeyCode::Up => *cursor -= (0, 1),
                KeyCode::Down => *cursor += (0, 1),
                KeyCode::Char('a') | KeyCode::Char('A') => ship_rot = ShipRotation::Left,
                KeyCode::Char('d') | KeyCode::Char('D') => ship_rot = ShipRotation::Right,
                KeyCode::Char('w') | KeyCode::Char('W') => ship_rot = ShipRotation::Up,
                KeyCode::Char('s') | KeyCode::Char('S') => ship_rot = ShipRotation::Down,
                KeyCode::Char(' ') | KeyCode::Enter => {
                    if board.ships.is_valid() && ship.next() {
                        break;
                    }
                    last_action_was_place = true;
                }
                _ => {}
            }
        }
        match ship {
            ShipType::AircraftCarrier => board.ships.carrier(ShipState::new(
                *cursor,
                ship_rot,
                false,
                ShipType::AircraftCarrier,
            )),
            ShipType::Battleship => board.ships.battleship(ShipState::new(
                *cursor,
                ship_rot,
                false,
                ShipType::Battleship,
            )),
            ShipType::Destroyer => board.ships.destroyer(ShipState::new(
                *cursor,
                ship_rot,
                false,
                ShipType::Destroyer,
            )),
            ShipType::Submarine => board.ships.submarine(ShipState::new(
                *cursor,
                ship_rot,
                false,
                ShipType::Submarine,
            )),
            ShipType::PatrolBoat => board.ships.patrol(ShipState::new(
                *cursor,
                ship_rot,
                false,
                ShipType::PatrolBoat,
            )),
        }
        if !board.ships.is_valid() && !last_action_was_place {
            message = "Invalid board layout".to_string();
        }
        render_board(stdout, &mut board, cursor, true, &message)?;
        message.clear();
        last_action_was_place = false;
    }
    *cursor = Cell::new(0, 0);
    Ok(board)
}

fn render_board(
    stdout: &mut Stdout,
    board: &mut Board,
    cursor: &mut Cell,
    ships: bool,
    message: &str,
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
            queue!(stdout, MoveTo((x + 1) * 3 - 1, y + 1))?;
            let cell = Cell::new(x.into(), y.into());
            let on_color = if ships && board.contains_ship(&cell) {
                Stylize::on_grey
            } else {
                Stylize::on_blue
            };
            match board.shot(&cell) {
                Shot::Hit => queue!(stdout, PrintStyledContent(on_color(" \u{25fe} ").red())),
                Shot::Miss => {
                    queue!(stdout, PrintStyledContent(on_color(" \u{25fe} ").white()))
                }
                Shot::Empty => queue!(stdout, PrintStyledContent(on_color("   "))),
            }?;
        }
    }
    queue!(
        stdout,
        MoveTo(0, 13),
        Print(message),
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
