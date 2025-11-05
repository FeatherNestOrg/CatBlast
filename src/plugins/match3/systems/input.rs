use crate::plugins::match3::components::{Gem, GridPosition, Selected};
use crate::plugins::match3::message::{GemClickedEvent, RequestSwapEvent};
use crate::plugins::match3::resources::{Board, Match3Config, SelectionState};
use crate::plugins::match3::systems::swap::check_swap;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

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
    info!("gem_input_system: mouse left pressed");
    let Ok(window) = q_window.single() else {
        warn!("gem_input_system: no primary window");
        return;
    };
    let Ok((camera, camera_transform)) = q_camera.single() else {
        warn!("gem_input_system: no Camera2d found");
        return;
    };

    if let Some(Ok(world_position)) = window
        .cursor_position()
        .map(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        let gem_size = config.gem_size;
        let board_size_x = board.width as f32 * gem_size;
        let board_size_y = board.height as f32 * gem_size;

        let board_origin_x = -board_size_x / 2.0;
        let board_origin_y = -board_size_y / 2.0;

        if world_position.x < board_origin_x
            || world_position.x > board_origin_x + board_size_x
            || world_position.y < board_origin_y
            || world_position.y > board_origin_y + board_size_y
        {
            return;
        }

        let grid_x = ((world_position.x - board_origin_x) / gem_size) as u32;
        let grid_y = ((world_position.y - board_origin_y) / gem_size) as u32;

        for (gem_entity, grid_pos) in gem_query.iter() {
            if grid_pos.x == grid_x && grid_pos.y == grid_y {
                debug!("Clicked on gem at ({}, {})", grid_x, grid_y);
                clicked_mw.write(GemClickedEvent { gem_entity });
                return;
            }
        }
    }
}

pub fn gem_selection_system(
    mut commands: Commands,
    mut selection_state: ResMut<SelectionState>,
    mut clicked_mr: MessageReader<GemClickedEvent>,
    mut swap_mw: MessageWriter<RequestSwapEvent>,
    mut board: ResMut<Board>,
    mut q_gems_pos: Query<&mut GridPosition>,
) {
    for event in clicked_mr.read() {
        let clicked_entity = event.gem_entity;
        match selection_state.selected_gem {
            None => {
                tracing::debug!("Selected first gem");
                selection_state.selected_gem = Some(clicked_entity);
                commands.entity(clicked_entity).insert(Selected::default());
            }
            Some(first_entity) => {
                if first_entity == clicked_entity {
                    debug!("Deselected gem.");
                    selection_state.selected_gem = None;
                    commands.entity(first_entity).remove::<Selected>();
                } else if !check_swap(&mut board, &mut q_gems_pos, clicked_entity, first_entity) {
                    debug!("Can't swap, select another");
                    selection_state.selected_gem = Some(clicked_entity);
                    commands.entity(clicked_entity).insert(Selected::default());
                    commands.entity(first_entity).remove::<Selected>();
                } else {
                    debug!("Selected second gem. Requesting swap.");

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
