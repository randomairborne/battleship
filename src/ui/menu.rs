use crate::board::{Board, Shot};
use crate::cell::Cell;
use crate::error::Error;
use crate::ship::{ShipRotation, ShipSetBuilder, ShipState, ShipType};
use crossterm::{
    cursor::MoveTo,
    event::{KeyCode, KeyModifiers},
    execute, queue,
    style::{Color, Print, PrintStyledContent, Stylize},
    terminal::Clear,
};
use std::io::Stdout;
