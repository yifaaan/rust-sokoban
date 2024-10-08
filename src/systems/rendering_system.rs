use std::time::Duration;

use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, Color, DrawParam, Drawable, Image},
    Context,
};
use specs::{Join, Read, ReadStorage, System};

use crate::{
    components::{Position, Renderable, RenderableKind},
    constants::TILE_WIDTH,
    resources::{GamePlay, Time},
};

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}

impl<'a> RenderingSystem<'a> {
    fn draw_text(&mut self, canvas: &mut Canvas, text_string: &str, x: f32, y: f32) {
        let text = graphics::Text::new(text_string);
        // let dimensions = Vec2::new(0.0, 20.0);
        let draw_param = DrawParam::new()
            .dest(Vec2::new(x, y))
            .color(Color::new(0.0, 0.0, 0.0, 1.0));
        canvas.draw(&text, draw_param);
    }

    /// 根据时间来判断播放的帧
    fn get_image(&mut self, renderable: &Renderable, delta: Duration) -> Image {
        let path_index = match renderable.kind() {
            RenderableKind::Static => 0,
            RenderableKind::Animated => {
                // 按250ms的间隔 播放每一帧
                ((delta.as_millis() % 1000) / 250) as usize
            }
        };
        let image_path = renderable.path(path_index);
        Image::from_path(self.context, image_path).expect("expected image")
    }
}

impl<'a> System<'a> for RenderingSystem<'a> {
    /// 渲染系统需要访问的数据
    type SystemData = (
        Read<'a, GamePlay>,
        Read<'a, Time>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (gameplay, time, positions, renderables) = data;
        // 清空背景
        let mut canvas =
            Canvas::from_frame(self.context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));

        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by_key(|&k| k.0.z);

        // Iterate through all pairs of positions & renderables, load the image
        // and draw it at the specified position.
        for (position, renderable) in rendering_data.iter() {
            // load image
            let image = self.get_image(renderable, time.delta);
            let x = position.x as f32 * TILE_WIDTH;
            let y = position.y as f32 * TILE_WIDTH;

            // draw
            let draw_params = DrawParam::new().dest(Vec2::new(x, y));
            canvas.draw(&image, draw_params);
        }
        // Render  text
        self.draw_text(&mut canvas, &gameplay.state.to_string(), 525.0, 80.0);
        self.draw_text(&mut canvas, &gameplay.move_count.to_string(), 525.0, 100.0);
        // present the context on the screen
        canvas.finish(self.context).expect("expected to present");
    }
}
