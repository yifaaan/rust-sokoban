use std::path;

use ggez::{
    conf::{self, WindowMode},
    event,
    glam::Vec2,
    graphics::{self, Canvas, DrawParam, Drawable, Image},
    input::{self, keyboard::KeyCode},
    Context, GameResult,
};
use specs::{prelude::*, storage, world, Component};

/// item 大小系数
const TILE_WIDTH: f32 = 30.0;
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

pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
}
/*
* create entity
*/
pub fn create_wall(world: &mut World, position: &Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..*position })
        .with(Renderable {
            path: "/images/wall.png".to_string(),
        })
        .with(Wall {})
        .build();
}

pub fn create_floor(world: &mut World, position: &Position) {
    world
        .create_entity()
        .with(Position { z: 5, ..*position })
        .with(Renderable {
            path: "/images/floor.png".to_string(),
        })
        .build();
}

pub fn create_box(world: &mut World, position: &Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..*position })
        .with(Renderable {
            path: "/images/box.png".to_string(),
        })
        .with(Box {})
        .build();
}

pub fn create_box_spot(world: &mut World, position: &Position) {
    world
        .create_entity()
        .with(Position { z: 9, ..*position })
        .with(Renderable {
            path: "/images/box_spot.png".to_string(),
        })
        .with(BoxSpot {})
        .build();
}

pub fn create_player(world: &mut World, position: &Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..*position })
        .with(Renderable {
            path: "/images/player.png".to_string(),
        })
        .with(Player {})
        .build();
}

pub fn load_map(world: &mut World, map_string: String) {
    let rows: Vec<&str> = map_string
        .trim()
        .split('\n')
        .map(|line| line.trim())
        .collect();

    for (y, row) in rows.iter().enumerate() {
        // println!("{}", row);
        let columns: Vec<&str> = row.split_whitespace().collect();
        for (x, column) in columns.iter().enumerate() {
            let position = Position {
                x: x as u8,
                y: y as u8,
                z: 0,
            };

            match *column {
                "." => create_floor(world, &position),
                "W" => {
                    create_floor(world, &position);
                    create_wall(world, &position);
                }
                "P" => {
                    create_floor(world, &position);
                    create_player(world, &position);
                }
                "B" => {
                    create_floor(world, &position);
                    create_box(world, &position);
                }
                "S" => {
                    create_floor(world, &position);
                    create_box_spot(world, &position);
                }
                "N" => (),
                c => panic!("unrecognized map item {}", c),
            }
        }
    }
}

/// Game State
struct Game {
    world: World,
}

impl event::EventHandler for Game {
    /// 每一帧调用该方法，用于更新游戏的状态
    fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult {
        {
            let mut is = InputSystem {};
            is.run_now(&self.world);
        }
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

    /// 键盘按下事件
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> GameResult {
        println!("Key pressed {:?}", input.keycode);

        let mut input_queue = self.world.write_resource::<InputQueue>();
        input_queue.keys_pressed.push(input.keycode.unwrap());
        Ok(())
    }
}

/// 作为world的全局共享资源
#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<KeyCode>,
}

pub struct RenderingSystem<'a> {
    context: &'a mut Context,
}

impl<'a> System<'a> for RenderingSystem<'a> {
    /// 渲染系统需要访问的数据
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);

    fn run(&mut self, data: Self::SystemData) {
        let (positions, renderables) = data;
        // 清空背景
        let mut canvas =
            Canvas::from_frame(self.context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));

        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by_key(|&k| k.0.z);

        // Iterate through all pairs of positions & renderables, load the image
        // and draw it at the specified position.
        for (position, renderable) in rendering_data.iter() {
            // load image
            let image =
                Image::from_path(self.context, renderable.path.clone()).expect("expect image");
            let x = position.x as f32 * TILE_WIDTH;
            let y = position.y as f32 * TILE_WIDTH;

            // draw
            let draw_params = DrawParam::new().dest(Vec2::new(x, y));
            canvas.draw(&image, draw_params);
        }
        // present the context on the screen
        canvas.finish(self.context).expect("expected to present");
    }
}

/// 初始化游戏关卡
pub fn initialize_level(world: &mut World) {
    // W:Wall  P: Player  B: Box  S: Spot  N: None
    const MAP: &str = "
    N N W W W W W W
    W W W . . . . W
    W . . . B . . W
    W . . . . . . W
    W . P . . . . W
    W . . . . . . W
    W . . S . . . W
    W . . . . . . W
    W W W W W W W W
    ";
    load_map(world, MAP.to_string());
    // create_player(world, &Position { x: 0, y: 0, z: 0 });
    // create_wall(world, &Position { x: 1, y: 0, z: 0 });
    // create_box(world, &Position { x: 2, y: 0, z: 0 });
}

pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Write<'a, InputQueue>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut input_queue, mut positions, players) = data;
        for (position, _player) in (&mut positions, &players).join() {
            if let Some(key) = input_queue.keys_pressed.pop() {
                match key {
                    KeyCode::Up => position.y -= 1,
                    KeyCode::Down => position.y += 1,
                    KeyCode::Left => position.x -= 1,
                    KeyCode::Right => position.x += 1,
                    _ => (),
                }
            }
        }
    }
}

fn main() -> GameResult {
    let mut world = World::new();
    register_components(&mut world);
    register_resources(&mut world);
    initialize_level(&mut world);

    let context_builder = ggez::ContextBuilder::new("rust_sokoban", "Liu Yifan")
        // 配置窗口的设置
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban"))
        // 缩放、无边框、全屏等
        .window_mode(WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path("./resources");

    let (context, event_loop) = context_builder.build()?;

    let game = Game { world };
    event::run(context, event_loop, game);
}
