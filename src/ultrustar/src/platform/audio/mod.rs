use std::fmt::Debug;

use anyhow::Result;

mod cpal;
use crate::SettingsTrait;

pub use tune::note::Note;

pub trait NoteInput {
    /// Extract the most recent note sample
    ///
    /// # Errors
    ///
    /// If there is an error with the `NoteInput` device
    fn read_current(&self) -> Result<Option<Note>>;
}

pub trait PlatformApi: Sized {
    type InitSettings: SettingsTrait;
    type NoteInputId: Debug;
    type NoteInput: NoteInput;

    /// Initialize the platform audio api
    ///
    /// # Errors
    ///
    /// Initialization may fail
    fn init(settings: &Self::InitSettings) -> Result<Self>;

    /// List available `NoteInput`s by their identifiers
    fn list_note_inputs(&self) -> Vec<Self::NoteInputId>;
    /// Create a new `NodeInput` device
    ///
    /// # Errors
    ///
    /// Creation of a device may fail
    fn create_note_input(&self, id: &Self::NoteInputId) -> Result<Self::NoteInput>;
}

pub use self::cpal::Platform;
