//! Song library
//!
//! Goals:
//! * Support for multiple loader types with dynamic availability depending on platform
//! * Persistable song library (cache for loader results)

use super::Song;
use anyhow::anyhow;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

/// Settings used for song library initialization
#[derive(Default, Serialize, Deserialize)]
pub struct Settings;
impl crate::SettingsTrait for Settings {}

/// Song metadata provided by `Loader`s' `crawl` functionality.
#[derive(Clone, Serialize, Deserialize)]
pub struct LoaderSong {
    /// General information about a song intended to help human users select a song
    infos: ultrastar_txt::structs::Header,
    /// Arbitrary data that can be used by loaders to allow for quicker re-identification of a LoaderSong returned previously.
    //
    // This could be a path to a file or a integer index...
    loader_key: String,
}

/// Global identifier for a loader
pub type LoaderId = &'static str;

/// Interface for `Loader` implementations.
pub trait Loader {
    fn loader_id(&self) -> LoaderId;
    fn crawl(&self) -> Vec<LoaderSong>;
    /// Load a song
    ///
    /// # Errors
    ///
    /// This operation may fail, e.g. if the original file location has become unavailable
    fn load(&self, song: &LoaderSong) -> Result<Song>;
}

/// A container type to represent a set of loaders to be used by a `Library`.
pub struct Loaders {
    loaders: Vec<Box<dyn Loader>>,
}
impl Loaders {
    fn builtin() -> Self {
        #[cfg(debug_assertions)]
        let loaders: Vec<Box<dyn Loader>> = vec![Box::new(devel::ExamplesLoader)];
        #[cfg(not(debug_assertions))]
        let loaders: Vec<Box<dyn Loader>> = vec![];
        Self { loaders }
    }
}
impl Deref for Loaders {
    type Target = [Box<dyn Loader>];

    fn deref(&self) -> &Self::Target {
        &self.loaders
    }
}

/// Wrapper around `LoaderSong` which only adds the information which loader it comes from.
pub struct LibrarySong {
    metadata: LoaderSong,
    loader: LoaderId,
}

/// Library of playable songs
pub struct Library {
    loaders: Loaders,
    songs: Vec<LibrarySong>,
}
impl Library {
    fn from_loaders(loaders: Loaders) -> Self {
        let songs: Vec<_> = loaders
            .iter()
            .flat_map(|loader| {
                loader
                    .crawl()
                    .drain(0..)
                    .map(|metadata| LibrarySong {
                        metadata,
                        loader: loader.loader_id(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        Self { loaders, songs }
    }

    #[must_use]
    pub fn init(_settings: &Settings) -> Self {
        Self::from_loaders(Loaders::builtin())
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.songs.len()
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.songs.is_empty()
    }

    /// Load the given song
    ///
    /// # Errors
    ///
    /// This operation may fail, e.g. if the original text file has disappeared in the meantime
    pub fn load(&self, song: &LibrarySong) -> Result<Song> {
        self.loaders
            .iter()
            .find(|l| l.loader_id() == song.loader)
            .ok_or_else(|| anyhow!("Failed to find loader {}", song.loader))
            .and_then(|loader| loader.load(&song.metadata))
    }
}
impl Deref for Library {
    type Target = [LibrarySong];

    fn deref(&self) -> &Self::Target {
        &self.songs
    }
}

#[cfg(debug_assertions)]
mod devel {
    use super::{Loader, LoaderSong, Result, Song};
    use anyhow::anyhow;

    const TXTS: [&str; 2] = [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/res/ultrastar-songs-libre-3/Joshua Morin - On the run/Joshua Morin - On the run.txt"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/res/ultrastar-songs-libre-3/Thor - Free Software Song/Thor - Free Software Song.txt"
        )),
    ];

    /// Test `Loader` implementation for development using the ultrastar-songs-libre-3 package checked into the codebase
    pub struct ExamplesLoader;
    impl Loader for ExamplesLoader {
        fn loader_id(&self) -> &'static str {
            "dev-examples"
        }

        fn crawl(&self) -> Vec<super::LoaderSong> {
            TXTS.iter()
                .copied()
                .map(ultrastar_txt::parse_txt_header_str)
                .map(Result::unwrap)
                .enumerate()
                .map(|(idx, infos)| LoaderSong {
                    infos,
                    loader_key: idx.to_string(),
                })
                .collect()
        }

        fn load(&self, song: &LoaderSong) -> Result<super::Song> {
            let idx: usize = song.loader_key.parse()?;
            let txtstr = TXTS
                .get(idx)
                .ok_or_else(|| anyhow!("Invalid index {}", idx))?;
            let header = ultrastar_txt::parse_txt_header_str(txtstr)
                .map_err(|err| anyhow!(err.to_string()))?;
            let lines = ultrastar_txt::parse_txt_lines_str(txtstr)
                .map_err(|err| anyhow!(err.to_string()))?;
            let txt = ultrastar_txt::TXTSong { header, lines };
            Ok(Song { txt })
        }
    }
}

#[cfg(test)]
mod test {
    use crate::model::{
        library::{devel::ExamplesLoader, LibrarySong, Loader, LoaderSong},
        Library,
    };

    use super::Loaders;

    #[test]
    fn example_lib() {
        let loaders = Loaders::builtin();
        assert_eq!(loaders.len(), 1);
        let library = Library::from_loaders(loaders);
        assert_eq!(2, library.len());
        let existing = &library[0];
        assert!(library.load(existing).is_ok());
        let missing_loader = LibrarySong {
            metadata: existing.metadata.clone(),
            loader: "foo",
        };
        assert!(library.load(&missing_loader).is_err());
        let missing_metadata = LoaderSong {
            infos: existing.metadata.infos.clone(),
            loader_key: "bar".into(),
        };
        let missing_song = LibrarySong {
            metadata: missing_metadata,
            loader: ExamplesLoader.loader_id(),
        };
        assert!(library.load(&missing_song).is_err());
    }
}
