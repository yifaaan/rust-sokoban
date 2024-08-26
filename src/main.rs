use std::path;

use ggez::{
    conf::{self, WindowMode},
    event, Context, GameResult,
};
use specs::{prelude::*, storage, Component};

#[derive(Debug, Clone, Component)]
#[storage(VecStorage)]
pub struct Position {
    x: u8,
    y: u8,
    z: u8,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    path: String,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Wall {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Player {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Box {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct BoxSpot {}

pub fn register_components(world: &mut World) {
    // The `World` is our
    // container for components
    // and other resources.
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Wall>();
    world.register::<Player>();
    world.register::<Box>();
    world.register::<BoxSpot>();
}

/*
* create entity
*/
pub fn create_wall(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            path: "images/wall.png".to_string(),
        })
        .with(Wall {})
        .build();
}

pub fn create_floor(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 5, ..position })
        .with(Renderable {
            path: "images/floor.png".to_string(),
        })
        .build();
}

pub fn create_box(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            path: "images/box.png".to_string(),
        })
        .with(Box {})
        .build();
}

pub fn create_box_spot(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 9, ..position })
        .with(Renderable {
            path: "images/box_spot.png".to_string(),
        })
        .with(BoxSpot {})
        .build();
}

pub fn create_player(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            path: "images/player.png".to_string(),
        })
        .with(Player {})
        .build();
}

/// Game State
struct Game {
    world: World,
}

impl event::EventHandler for Game {
    /// 每一帧调用该方法，用于更新游戏的状态
    fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult {
        Ok(())
    }
    /// 每一帧调用该方法，用于绘制游戏内容
    fn draw(&mut self, _ctx: &mut ggez::Context) -> GameResult {
        {
            // 在每一帧渲染更新所有实体的状态
            let mut rs = RenderingSystem { context: _ctx };
            rs.run_now(&self.world);
        }
        Ok(())
    }
}

pub struct RenderingSystem<'a> {
    context: &'a mut Context,
}

impl<'a> System<'a> for RenderingSystem<'a> {
    /// 渲染系统需要访问的数据
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);

    fn run(&mut self, data: Self::SystemData) {
        let (positions, renderables) = data;
        unimplemented!();
    }
}

fn main() -> GameResult {
    let mut world = World::new();
    register_components(&mut world);

    let context_builder = ggez::ContextBuilder::new("rust_sokoban", "lyf")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban"))
        .window_mode(WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (context, event_loop) = context_builder.build()?;

    let game = Game { world };
    event::run(context, event_loop, game);
}
