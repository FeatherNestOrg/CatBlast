use crate::GameState;
use bevy::prelude::*;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, SubStates)]
#[source(GameState = GameState::Match3)]
pub enum Match3State {
    #[default]
    AwaitingInput,
    Animating,
    ProcessingMatches,
}
