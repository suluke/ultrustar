use anyhow::Result;

mod cpal;
use crate::SettingsTrait;

pub trait NoteInput {}

pub trait PlatformApi: Sized {
    type InitSettings: SettingsTrait;
    type NoteInputId;
    type NoteInput: NoteInput;

    /// Initialize the platform audio api
    ///
    /// # Errors
    ///
    /// Initialization may fail
    fn init(settings: Self::InitSettings) -> Result<Self>;

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
