#![allow(dead_code)]

pub mod library;
pub use library::Library;

///
pub type Score = f32;

///
pub struct Song {
    txt: ultrastar_txt::structs::TXTSong,
}
impl Song {
    fn score() -> Score {
        1.0f32
    }
}

/// Interface for subsystems that "accompany" a song while it's played.
//
// Examples would be (obviously) sound files, music videos, etc.
trait Accompaniment {}

///
pub struct Player;

///
pub struct GameState;
