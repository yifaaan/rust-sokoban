use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, DrawParam, Image},
    Context,
};
use specs::{Join, ReadStorage, System};

use crate::{
    components::{Position, Renderable},
    constants::TILE_WIDTH,
};

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
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
