use crate::components::Position;
use crate::entities::*;
use specs::World;

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
