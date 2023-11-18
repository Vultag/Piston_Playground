

use bevy::ecs::bundle;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

//use std::thread::spawn;

//use bevy::{transform::commands, prelude::{Camera2dBundle, App, Startup}, utils::default, DefaultPlugins};


fn main() 
{
    
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup,spawn_cam)
        .run()

}


fn spawn_cam(mut commands: Commands,) {

    commands.spawn(Camera2dBundle::default());

    
}

#[derive(Component)]
pub struct Player{}


pub fn spawn_player
    (
        mut commands: Commands,
        window_query : Query<&Window, with<PrimaryWindow>>,
        asset_server : Res<AssetServer>,
    ) 
    {
        let window: &Window = window_query.get_single().unwrap();

        commands.spawn(bundle:(
            SpriteBundle{
                transform: Transform::from_xyz(window.width()*0.5, window.height()*0.5, 0),
                texture: asset_server.load("Sprites/Vowsh"),
                ..default()
                
            },
            Player{},
        ));
    
    }


