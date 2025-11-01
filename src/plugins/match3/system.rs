use crate::plugins::match3::components::{Gem, GridPosition, Selected};
use crate::plugins::match3::message::{GemClickedEvent, RequestSwapEvent};
use crate::plugins::match3::resources::{Board, Match3Config, SelectionState};
use bevy::camera::{Camera, Camera2d};
use bevy::input::ButtonInput;
use bevy::prelude::{Commands, Entity, GlobalTransform, MessageReader, MessageWriter, MouseButton, Query, Res, ResMut, With};
use bevy::window::{PrimaryWindow, Window};

pub fn gem_input_system(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    board: Res<Board>,
    gem_query: Query<(Entity, &GridPosition), With<Gem>>,
    mut clicked_mw: MessageWriter<GemClickedEvent>,
    config: Res<Match3Config>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }
    let Ok(window) = q_window.single() else {
        return;
    };
    let Ok((camera, camera_transform)) = q_camera.single() else {
        return;
    };

    if let Some(Ok(world_position)) = window.cursor_position().map(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)) {
        let gem_size = config.gem_size;
        let board_size_x = board.width as f32 * gem_size;
        let board_size_y = board.height as f32 * gem_size;

        let board_origin_x = -board_size_x / 2.0;
        let board_origin_y = -board_size_y / 2.0;

        if world_position.x < board_origin_x || world_position.x > board_origin_x + board_size_x ||
            world_position.y < board_origin_y || world_position.y > board_origin_y + board_size_y {
            return;
        }

        let grid_x = ((world_position.x - board_origin_x) / gem_size) as u32;
        let grid_y = ((world_position.y - board_origin_y) / gem_size) as u32;

        for (gem_entity, grid_pos) in gem_query.iter() {
            if grid_pos.x == grid_x && grid_pos.y == grid_y {
                // 找到了！发送点击事件
                println!("Clicked on gem at ({}, {})", grid_x, grid_y);
                clicked_mw.write(GemClickedEvent { gem_entity });
                return; // 找到后即可退出
            }
        }
    }
}

pub fn gem_selection_system(
    mut commands: Commands,
    mut selection_state: ResMut<SelectionState>,
    mut clicked_mr: MessageReader<GemClickedEvent>,
    mut swap_mw: MessageWriter<RequestSwapEvent>,
    q_selected: Query<Entity, With<Selected>>,
) {
    for event in clicked_mr.read() {
        let clicked_entity = event.gem_entity;
        match selection_state.selected_gem {
            None => {
                println!("Selected first gem");
                selection_state.selected_gem = Some(clicked_entity);
                commands.entity(clicked_entity).insert(Selected);
            }
            Some(first_entity) => {
                if first_entity == clicked_entity {
                    println!("Deselected gem.");
                    selection_state.selected_gem = None;
                    commands.entity(first_entity).remove::<Selected>();
                } else {
                    println!("Selected second gem. Requesting swap.");

                    swap_mw.write(RequestSwapEvent {
                        entity1: first_entity,
                        entity2: clicked_entity,
                    });
                    commands.entity(first_entity).remove::<Selected>();
                    selection_state.selected_gem = None;
                }
            }
        }
    }
}