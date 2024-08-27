use components::register_components;
use ggez::{
    conf::{self, WindowMode},
    event, timer, Context, GameResult,
};
use map::load_map;
use resources::{register_resources, InputQueue, Time};
use specs::prelude::*;
use systems::{GamePlayStateSystem, InputSystem, RenderingSystem};

mod components;
mod constants;
mod entities;
mod map;
mod resources;
mod systems;
/// item 大小系数

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
        // Run gameplay state system
        {
            let mut gss = GamePlayStateSystem {};
            gss.run_now(&self.world);
        }

        {
            let mut time = self.world.write_resource::<Time>();
            // 加上每一帧的时间间隔
            time.delta += _ctx.time.delta();
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

/// 初始化游戏关卡
pub fn initialize_level(world: &mut World) {
    // W:Wall  P: Player  B: Box  S: Spot  N: None
    const MAP: &str = "
    N N W W W W W W
    W W W . . . . W
    W . . . BB . . W
    W . . RB . . . W 
    W . P . . . . W
    W . . . . RS . W
    W . . BS . . . W
    W . . . . . . W
    W W W W W W W W
    ";
    load_map(world, MAP.to_string());
    // create_player(world, &Position { x: 0, y: 0, z: 0 });
    // create_wall(world, &Position { x: 1, y: 0, z: 0 });
    // create_box(world, &Position { x: 2, y: 0, z: 0 });
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
