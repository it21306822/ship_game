use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;
use crate::component::{BoxEntity, BoxDirection, Ship, StartPoint, EndPoint, Fireball};

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    // Spawn the camera
    commands.spawn(Camera2dBundle::default());

    // Load the ship and box textures
    let ship_handle = asset_server.load("ship.png");
    let box_handle = asset_server.load("box.png");

    // Get the primary window
    if let Ok(window) = windows.get_single() {
        let margin = 20.0; // Define a margin from the window edges
        let half_width = window.width() / 2.0;
        let half_height = window.height() / 2.0;

        // Spawn the start point visual (e.g., a green square)
        commands.spawn(( 
            SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(-half_width + margin, half_height - margin, 0.0), // Top-left corner with margin
                    scale: Vec3::new(20.0, 20.0, 1.0), // Larger size for the start point visual
                    ..Default::default()
                },
                ..Default::default()
            },
            StartPoint,
        ));

        // Spawn the end point visual (e.g., a red square)
        commands.spawn(( 
            SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(half_width - margin, -half_height + margin, 0.0), // Bottom-right corner with margin
                    scale: Vec3::new(20.0, 20.0, 1.0), // Larger size for the end point visual
                    ..Default::default()
                },
                ..Default::default()
            },
            EndPoint,
        ));

        // Spawn the ship near the start point (right side)
        commands.spawn(( 
            SpriteBundle {
                texture: ship_handle,
                transform: Transform {
                    translation: Vec3::new(-half_width + margin + 40.0, half_height - margin - 40.0, 0.0), // Right side of the top-left corner
                    scale: Vec3::new(0.1, 0.1, 1.0), // Smaller size for the ship
                    rotation: Quat::from_rotation_z(0.0), // Initial rotation
                    ..Default::default()
                },
                ..Default::default()
            },
            Ship,
        ));

        // Spawn 5 boxes at random positions
        let num_boxes = 5;
        let mut rng = rand::thread_rng(); // Create a random number generator
        for _ in 0..num_boxes {
            let x = rng.gen_range(-half_width + margin..half_width - margin);
            let y = rng.gen_range(-half_height + margin..half_height - margin);

            // Random initial direction
            let direction = Vec3::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                0.0,
            ).normalize_or_zero();

            commands.spawn(( 
                SpriteBundle {
                    texture: box_handle.clone(),
                    transform: Transform {
                        translation: Vec3::new(x, y, 0.0),
                        scale: Vec3::new(0.2, 0.2, 1.0), // Adjust size as needed
                        ..Default::default()
                    },
                    ..Default::default()
                },
                BoxEntity,
                BoxDirection(direction),
            ));
        }
    }
}

pub fn box_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut BoxDirection), With<BoxEntity>>,
) {
    for (mut transform, mut direction) in query.iter_mut() {
        let speed = 50.0; // Reduced speed for smoother movement
        let movement = Vec3::new(speed * time.delta_seconds(), speed * time.delta_seconds(), 0.0);

        // Move the box in the current direction
        transform.translation += direction.0 * movement;

        // Optionally, handle boundary collisions or change direction randomly
        let half_width = 400.0; // Adjust this based on your window size or requirements
        let half_height = 300.0; // Adjust this based on your window size or requirements
        
        if transform.translation.x.abs() > half_width || transform.translation.y.abs() > half_height {
            // Reverse direction when hitting boundaries
            direction.0 *= -1.0;
        }
    }
}

pub fn fireball_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Fireball)>,
) {
    for (mut transform, _) in query.iter_mut() {
        let speed = 300.0;
        transform.translation += Vec3::new(speed * time.delta_seconds(), 0.0, 0.0);
        
        // Optionally, you can add logic to remove fireballs when they go off-screen or hit an object
    }
}
