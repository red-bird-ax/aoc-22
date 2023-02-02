use std::ffi::{
    CString, c_char
};

pub struct Iterator {
    handle: *mut c_iter::GuideIterator,
}

#[derive(Copy, Clone, PartialEq)]
pub enum HandShape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Copy, Clone)]
pub struct Round {
    pub player: HandShape,
    pub opponent: HandShape,
}

impl Iterator {
    pub fn new(path: &str) -> Result<Iterator, String> {
        let c_path = match CString::new(path) {
            Ok(path) => path,
            Err(err) => return Err(err.to_string()),
        };

        unsafe {
            let handle = c_iter::NewIterator(c_path.into_raw());
            if handle.is_null() {
                Err(String::from("failed to create iterator"))
            } else {
                Ok(Iterator{ handle })
            }
        }
    }

    pub fn next(&self) -> Result<Option<Round>, String> {
        let moves: (c_char, c_char);
        unsafe {
            let round = c_iter::Next(self.handle);
            if c_iter::IsValidRound(round) == 0 {
                return Ok(None);
            }
            moves = (round.player, round.opponent);
        }

        let player = match parse_hand_shape(moves.0) {
            Ok(shape) => shape,
            Err(msg) => return Err(format!("player shape: {}", msg)),
        };

        let opponent = match parse_hand_shape(moves.1) {
            Ok(shape) => shape,
            Err(msg) => return Err(format!("opponent shape: {}", msg)),
        };

        Ok(Some(Round{ player, opponent }))
    }
}

impl Drop for Iterator {
    fn drop(&mut self) {
        unsafe {
            c_iter::DestroyIterator(self.handle);
        }
    }
}

impl Round {
    pub fn play(self) -> i32 {
        self.get_player_score() + self.get_round_score()
    }

    fn get_player_score(self) -> i32 {
        match self.player {
            HandShape::Rock => 1,
            HandShape::Paper => 2,
            HandShape::Scissors => 3,
        }
    }

    fn get_round_score(self) -> i32 {
        if self.player == self.opponent {
            return 3;
        }
        match (self.player, self.opponent) {
            (HandShape::Rock, HandShape::Scissors) |
            (HandShape::Paper, HandShape::Rock) |
            (HandShape::Scissors, HandShape::Paper) => 6,
            _ => 0,
        }
    }
}

fn parse_hand_shape(symbol: c_char) -> Result<HandShape, String> {
    let symbol = match std::char::from_u32(symbol as u32) {
        Some(symbol) => symbol,
        None => return Err(String::from("can`t parse symbol from iterator")),
    };

    match symbol {
        'A' | 'X' => Ok(HandShape::Rock),
        'B' | 'Y' => Ok(HandShape::Paper),
        'C' | 'Z' => Ok(HandShape::Scissors),
        _ => Err(format!("invalid move symbol: {}", symbol))
    }
}

mod c_iter{
    use std::ffi::c_char;

    #[link(name = "iterator", kind = "static")]
    extern "C" {
        pub fn NewIterator(path: *const c_char) -> *mut GuideIterator;
        pub fn DestroyIterator(it: *mut GuideIterator);
        pub fn Next(it: *mut GuideIterator) -> Round;
        pub fn IsValidRound(round: Round) -> c_char;
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Round {
        pub opponent: c_char,
        pub player: c_char,
    }

    #[repr(C)]
    pub struct GuideIterator {
        placeholder: [i8; 56],
    }
}