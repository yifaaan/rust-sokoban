use std::collections::HashMap;

use crate::components::*;
use crate::constants::{MAP_HEIGHT, MAP_WIDTH};
use crate::resources::*;
use ggez::input::keyboard::KeyCode;
use specs::{Entities, Join, ReadStorage, System, Write, WriteStorage};

pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Write<'a, InputQueue>,
        Write<'a, GamePlay>,
        Entities<'a>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Movable>,
        ReadStorage<'a, Immovable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut input_queue,
            mut game_play,
            entities,
            mut positions,
            players,
            movables,
            immovables,
        ) = data;

        let mut to_move = Vec::new();
        for (position, _player) in (&positions, &players).join() {
            if let Some(key) = input_queue.keys_pressed.pop() {
                // (x, y) --> id
                let mov = (&entities, &movables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();
                let immov = (&entities, &immovables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();

                // 在该方向从起点迭代到地图的边界
                let (start, end, is_x) = match key {
                    KeyCode::Up => (position.y, 0, false),
                    KeyCode::Down => (position.y, MAP_HEIGHT, false),
                    KeyCode::Left => (position.x, 0, true),
                    KeyCode::Right => (position.x, MAP_WIDTH, true),
                    _ => continue,
                };

                let range = if start < end {
                    (start..=end).collect::<Vec<_>>()
                } else {
                    (end..=start).rev().collect::<Vec<_>>()
                };

                // 遍历路径上的每一个点
                for x_or_y in range {
                    let pos = if is_x {
                        (x_or_y, position.y)
                    } else {
                        (position.x, x_or_y)
                    };

                    match mov.get(&pos) {
                        // entity
                        Some(id) => to_move.push((key, id.clone())),
                        None => {
                            // immovable
                            match immov.get(&pos) {
                                // 如果路径上有一个Immovable，则停止移动所有
                                Some(_id) => to_move.clear(),
                                // floor
                                None => break,
                            }
                        }
                    }
                }
            }
        }
        // increase the number of moves
        if to_move.len() > 0 {
            game_play.move_count += 1;
        }
        // 移动所有可移动物
        for (key, id) in to_move {
            let position = positions.get_mut(entities.entity(id));
            if let Some(p) = position {
                match key {
                    KeyCode::Up => p.y -= 1,
                    KeyCode::Down => p.y += 1,
                    KeyCode::Left => p.x -= 1,
                    KeyCode::Right => p.x += 1,
                    _ => (),
                }
            }
        }
    }
}
