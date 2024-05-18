use bevy::asset::{AssetLoader, AsyncReadExt};
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use serde::Deserialize;
use thiserror::Error;

fn main() {
    App::new()
        // plugins
        .add_plugins((DefaultPlugins, WorldInspectorPlugin::default()))
        // asset stuff
        .init_asset::<TestAsset>()
        .register_asset_reflect::<TestAsset>()
        .register_asset_loader(TestAssetLoader)
        // systems
        .add_systems(Startup, setup)
        .add_systems(Update, asset_event_handler)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let handle: Handle<TestAsset> = asset_server.load("test.ron");

    commands.spawn((Name::from("Test entity!".to_string()), handle));
}

fn asset_event_handler(
    mut events: EventReader<AssetEvent<TestAsset>>,
    test_assets: Res<Assets<TestAsset>>,
) {
    for event in events.read() {
        match event {
            AssetEvent::Added { id } => {
                let test_asset = test_assets.get(*id).unwrap();
                info!("Added: {test_asset:?}");
            }
            AssetEvent::Modified { id } => {
                let test_asset = test_assets.get(*id).unwrap();
                info!("Modified: {test_asset:?}");
            }
            AssetEvent::Removed { id: _ }
            | AssetEvent::Unused { id: _ }
            | AssetEvent::LoadedWithDependencies { id: _ } => {}
        }
    }
}

#[derive(Asset, Debug, Deserialize, Reflect)]
struct TestAsset {
    test_field: String,
}

#[derive(Debug, Error)]
enum TestAssetLoaderError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    RonSpannedError(#[from] ron::error::SpannedError),
}

struct TestAssetLoader;

impl AssetLoader for TestAssetLoader {
    type Asset = TestAsset;

    type Settings = ();

    type Error = TestAssetLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let ron: TestAsset = ron::de::from_bytes(&bytes)?;

            Ok(ron)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["test.ron"]
    }
}
