use std::collections::HashMap;

use specs::{Join, ReadStorage, System, Write};

use crate::{
    components::{Box, BoxSpot, Position},
    resources::{GamePlay, GamePlayState},
};

pub struct GamePlayStateSystem {}

impl<'a> System<'a> for GamePlayStateSystem {
    type SystemData = (
        Write<'a, GamePlay>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Box>,
        ReadStorage<'a, BoxSpot>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut gameplay, positions, boxes, boxspots) = data;
        // get all boxes indexed by position
        let boxes_by_position = (&positions, &boxes)
            .join()
            .map(|t| ((t.0.x, t.0.y), t.1))
            .collect::<HashMap<_, _>>();
        //
        for (_box_spot, position) in (&boxspots, &positions).join() {
            // 只要还存在没有归位的box，就return，游戏继续
            if !boxes_by_position.contains_key(&(position.x, position.y)) {
                gameplay.state = GamePlayState::Playing;
                return;
            }
        }
        // 所有box都归位，游戏获胜
        gameplay.state = GamePlayState::Won;
    }
}
