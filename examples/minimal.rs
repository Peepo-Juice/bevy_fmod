//! This example demonstrates how to use the FmodPlugin to play a sound.
//! Make sure to follow the instructions in the README.md to set up the demo project.

use bevy::prelude::*;
use bevy_fmod::prelude::AudioSource;
use bevy_fmod::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FmodPlugin {
                audio_banks_paths: &[
                    "./assets/audio/demo_project/Build/Desktop/Master.bank",
                    "./assets/audio/demo_project/Build/Desktop/Master.strings.bank",
                    "./assets/audio/demo_project/Build/Desktop/Music.bank",
                ],
            },
        ))
        .add_systems(Startup, startup)
        .add_systems(PostStartup, play_music)
        .run();
}

#[derive(Component)]
struct MyMusicPlayer;

fn startup(mut commands: Commands, studio: Res<FmodStudio>) {
    commands
        .spawn(MyMusicPlayer)
        .insert(studio.build_audio_source(
            "event:/Music/Level 03",
            libfmod::StopMode::Immediate,
            false,
        ));
}

fn play_music(mut audio_sources: Query<&AudioSource, With<MyMusicPlayer>>) {
    audio_sources.single_mut().play();
}
