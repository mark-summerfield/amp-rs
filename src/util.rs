// Copyright © 2021 Mark Summerfield. All rights reserved.
// License: GPLv3

use std::{cmp, fmt, str};

pub fn x() -> i32 {
    (fltk::app::screen_size().0 / 2.0) as i32
}

pub fn y() -> i32 {
    (fltk::app::screen_size().1 / 2.0) as i32
}

pub fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

// Returns a number converted from the given str or the default
pub fn get_num<T>(s: &str, minimum: T, maximum: T, default: T) -> T
where
    T: num::Num + cmp::PartialOrd + Copy + str::FromStr,
{
    match s.parse() {
        Ok(n) if minimum <= n && n <= maximum => n,
        _ => default,
    }
}


pub fn isclose32(a: f32, b: f32) -> bool {
    (a..=(a + f32::EPSILON)).contains(&b)
}

pub fn isone32(n: f32) -> bool {
    (1.0..=(1.0 + f32::EPSILON)).contains(&n)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub const INVALID: i32 = -1;

    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn is_valid(&self) -> bool {
        self.x != Pos::INVALID && self.y != Pos::INVALID
    }
}

impl Default for Pos {
    fn default() -> Self {
        Self { x: Pos::INVALID, y: Pos::INVALID }
    }
}

impl std::fmt::Display for Pos {
    fn fmt(&self, out: &mut std::fmt::Formatter) -> fmt::Result {
        write!(out, "({},{})", self.x, self.y)
    }
}
