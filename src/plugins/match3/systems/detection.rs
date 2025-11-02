use crate::plugins::match3::state::Match3State;
use bevy::prelude::*;

pub fn match_detection_system(mut next_state: ResMut<NextState<Match3State>>) {
    next_state.set(Match3State::AwaitingInput)
}
