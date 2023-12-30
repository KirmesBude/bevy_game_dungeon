use serde::Deserialize;
use thiserror::Error;

use crate::movement::GridPosition;

use super::{interactables::Interactable, Tile};

use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    reflect::TypePath,
    utils::{BoxedFuture, HashMap},
};

#[derive(Debug, Default, Deserialize, Asset, TypePath)]
pub struct Level {
    pub grid: Vec<Vec<Tile>>,
    pub start_pos: GridPosition,
    #[serde(default)]
    pub interactables: HashMap<GridPosition, Interactable>,
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum LevelAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [RON](ron) Error
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
}

#[derive(Default)]
pub struct LevelAssetLoader;

impl AssetLoader for LevelAssetLoader {
    type Asset = Level;
    type Settings = ();
    type Error = LevelAssetLoaderError;
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let custom_asset = ron::de::from_bytes::<Level>(&bytes)?;
            Ok(custom_asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["lvl"]
    }
}
