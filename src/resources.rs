use ggez::input::keyboard::KeyCode;
use specs::World;

/// 作为world的全局共享资源
#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<KeyCode>,
}
pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
}
