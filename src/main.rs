
mod components;
mod auxfcns;

use bevy::prelude::*;
use std::collections::VecDeque;
use crate::components::*;
use crate::auxfcns::*;


fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(animate_sprite_system.system())
        .add_system(movement_system.system())
        .run();
}

fn animate_sprite_system(
    mut query: Query<(
        &Timer, 
        &mut TextureAtlasSprite, 
        &FacingDirection, 
        &Player, 
        &mut AnimationStep,
        &Animate
    )>,
) {
    for (
        timer, 
        mut sprite, 
        facing, 
        _player, 
        mut step,
        animate_flag,
    ) in &mut query.iter() {
        if timer.finished && animate_flag.0 { // or keyboard.just_pressed(arrowkey)
            // Yikes! Magic numbers.
            // Animation length = 3
            // Row length on spritesheet = 12
            step.0 = (step.0 + 1) % 3;
            if !facing.0.is_empty() {
                let offset = 12 * facing_to_row(&facing.0.front().unwrap());
                sprite.index = (offset + step.0) as u32;            
            }
        }
    }
}

// Abstract these into some kind of file to be read at runtime. Or changed at some point ingame.
const KEY_MOVE_NORTH: KeyCode = KeyCode::Up;
const KEY_MOVE_SOUTH: KeyCode = KeyCode::Down;
const KEY_MOVE_WEST: KeyCode = KeyCode::Left;
const KEY_MOVE_EAST: KeyCode = KeyCode::Right;

fn mmnt_key_up(
    movement: &mut Mut<MovementQueue>, 
    facing: &mut Mut<FacingDirection>, 
    dir: Facing) {
    movement.0.retain(|&v| v != dir);
    facing.0.retain(|&v| v != dir);
}

fn movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        &Player, 
        &mut Animate,
        &mut Transform,
        &Speed,
        &mut MovementQueue,
        &mut Timer,
        &mut FacingDirection,
    )>
) {
    for (
        _player,
        mut animate_flag, 
        mut transform,
        speed,
        mut movement,
        mut timer,
        mut facing
    ) in &mut query.iter() {
        if keyboard_input.just_pressed(KEY_MOVE_NORTH) {
            (*movement).0.push_back(Facing::North);
            (*facing).0.push_back(Facing::North);
            timer.reset();
        } else if keyboard_input.just_pressed(KEY_MOVE_WEST) {
            (*movement).0.push_back(Facing::West);
            (*facing).0.push_back(Facing::West);
            timer.reset();
        } else if keyboard_input.just_pressed(KEY_MOVE_SOUTH) {
            (*movement).0.push_back(Facing::South);
            (*facing).0.push_back(Facing::South);
            timer.reset();
        } else if keyboard_input.just_pressed(KEY_MOVE_EAST) {
            (*movement).0.push_back(Facing::East);
            (*facing).0.push_back(Facing::East);
            timer.reset();
        }

        if keyboard_input.pressed(KEY_MOVE_NORTH) ||
           keyboard_input.pressed(KEY_MOVE_WEST)  ||
           keyboard_input.pressed(KEY_MOVE_SOUTH) ||
           keyboard_input.pressed(KEY_MOVE_EAST)
        {
            (*animate_flag).0 = true;
        }

        let translation = transform.translation_mut();

        if !movement.0.is_empty() {
            match movement.0.front().unwrap() {
                Facing::North => *translation.y_mut() += (*speed).0,
                Facing::East => *translation.x_mut() += (*speed).0,
                Facing::South => *translation.y_mut() -= (*speed).0,
                Facing::West => *translation.x_mut() -= (*speed).0,
            }
        }

        // This thing needs to be a match thing instead i think.
        // After some digging, not sure it can be as gracefully as I would like.
        // You _can_ get an iterator from keyboard_input via .get_pressed().
        if keyboard_input.just_released(KEY_MOVE_NORTH) {
            (*animate_flag).0 = false;
            mmnt_key_up(&mut movement, &mut facing, Facing::North);
        } else if keyboard_input.just_released(KEY_MOVE_WEST) {
            (*animate_flag).0 = false;
            mmnt_key_up(&mut movement, &mut facing, Facing::West);
        } else if keyboard_input.just_released(KEY_MOVE_SOUTH) {
            (*animate_flag).0 = false;  
            mmnt_key_up(&mut movement, &mut facing, Facing::South);
        } else if keyboard_input.just_released(KEY_MOVE_EAST) {
            (*animate_flag).0 = false;
            mmnt_key_up(&mut movement, &mut facing, Facing::East);
        }

    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server
        .load_sync(
            &mut textures,
            "assets/bardo.png",
        )
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 12, 8);

    let player_position = Vec2::new(0.0, 0.0);
    let mut player_init_transform = Transform::from_scale(2.0);
    player_init_transform.set_translation(Vec3::new(player_position.x(), player_position.y(), 0.0));

    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            transform: player_init_transform,
            ..Default::default()
        })
        .with(Player)
        .with(Timer::from_seconds(0.1, true))
        .with(Facing::West)
        .with(Animate(false))
        .with(Speed(0.5))
        .with(MovementQueue(VecDeque::new()))
        .with(FacingDirection(VecDeque::new()))
        .with(AnimationStep(0));
}