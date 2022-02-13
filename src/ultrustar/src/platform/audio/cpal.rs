use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct InitSettings;
impl crate::SettingsTrait for InitSettings {}

pub struct NoteInput;
impl super::NoteInput for NoteInput {}

pub struct Platform;

impl super::PlatformApi for Platform {
    type InitSettings = InitSettings;

    type NoteInputId = ();

    type NoteInput = NoteInput;

    fn init(_settings: Self::InitSettings) -> anyhow::Result<Self> {
        Ok(Self)
    }

    fn list_note_inputs(&self) -> Vec<Self::NoteInputId> {
        vec![()]
    }

    fn create_note_input(&self, _id: &Self::NoteInputId) -> anyhow::Result<Self::NoteInput> {
        Ok(NoteInput)
    }
}
