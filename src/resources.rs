use std::fmt::Display;

use ggez::input::keyboard::KeyCode;
use specs::World;

/// 作为world的全局共享资源
#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<KeyCode>,
}

#[derive(Default)]
pub struct GamePlay {
    pub state: GamePlayState,
    pub move_count: u32,
}

pub enum GamePlayState {
    Playing,
    Won,
}

impl Default for GamePlayState {
    fn default() -> Self {
        Self::Playing
    }
}

impl Display for GamePlayState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GamePlayState::Playing => "Playing",
                GamePlayState::Won => "Won",
            }
        )
    }
}

pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
    world.insert(GamePlay::default());
}
