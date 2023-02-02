use std::ffi::{
    CString, c_char
};

pub struct Iterator {
    handle: *mut c_iter::GuideIterator,
}

#[derive(Copy, Clone, PartialEq)]
pub enum HandShape { Rock, Paper, Scissors }

#[derive(Copy, Clone)]
enum RoundResult { Win, Loose, Draw }

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

        let player = match Iterator::parse_hand_shape(moves.0) {
            Ok(shape) => shape,
            Err(msg) => return Err(format!("player shape: {}", msg)),
        };

        let opponent = match Iterator::parse_hand_shape(moves.1) {
            Ok(shape) => shape,
            Err(msg) => return Err(format!("opponent shape: {}", msg)),
        };

        Ok(Some(Round{ player, opponent }))
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
}

impl Drop for Iterator {
    fn drop(&mut self) {
        unsafe {
            c_iter::DestroyIterator(self.handle);
        }
    }
}

impl Round {
    fn get_move_score(shape: HandShape) -> i32 {
        match shape {
            HandShape::Rock => 1,
            HandShape::Paper => 2,
            HandShape::Scissors => 3,
        }
    }

    fn get_score(result: RoundResult) -> i32 {
        match result {
            RoundResult::Loose => 0,
            RoundResult::Win => 6,
            RoundResult::Draw => 3,
        }
    }

    pub fn play_part_1(self) -> i32 {
        let result = self.get_round_result_part_1();
        Round::get_move_score(self.player) + Round::get_score(result)
    }

    fn get_round_result_part_1(self) -> RoundResult {
        if self.player == self.opponent {
            return RoundResult::Draw
        }
        match (self.player, self.opponent) {
            (HandShape::Rock, HandShape::Scissors) |
            (HandShape::Paper, HandShape::Rock) |
            (HandShape::Scissors, HandShape::Paper) => RoundResult::Win,
            _ => RoundResult::Loose,
        }
    }

    pub fn play_part_2(self) -> i32 {
        let result = self.get_round_result_part2();
        let player = self.get_player_move(result);

        Round::get_move_score(player) + Round::get_score(result)
    }

    fn get_round_result_part2(self) -> RoundResult {
        match self.player {
            HandShape::Rock => RoundResult::Loose,
            HandShape::Paper => RoundResult::Draw,
            HandShape::Scissors => RoundResult::Win,
        }
    }

    fn get_player_move(self, result: RoundResult) -> HandShape {
        match result {
            RoundResult::Loose => {
                match self.opponent {
                    HandShape::Rock => HandShape::Scissors,
                    HandShape::Paper => HandShape::Rock,
                    HandShape::Scissors => HandShape::Paper,
                }
            },
            RoundResult::Win => {
                match self.opponent {
                    HandShape::Rock => HandShape::Paper,
                    HandShape::Paper => HandShape::Scissors,
                    HandShape::Scissors => HandShape::Rock,
                }
            },
            RoundResult::Draw => self.opponent
        }
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