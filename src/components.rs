use std::fmt::Display;

use specs::{Component, NullStorage, VecStorage, World, WorldExt};

#[derive(Debug, Clone, Component)]
#[storage(VecStorage)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    pub paths: Vec<String>,
}

impl Renderable {
    /// 静态渲染对象
    pub fn new_static(path: String) -> Self {
        Self { paths: vec![path] }
    }
    /// 动态渲染对象
    pub fn new_animated(paths: Vec<String>) -> Self {
        Self { paths }
    }
    /// 渲染类型
    pub fn kind(&self) -> RenderableKind {
        match self.paths.len() {
            0 => panic!("invalid renderable"),
            1 => RenderableKind::Static,
            _ => RenderableKind::Animated,
        }
    }

    /// 根据索引找路径
    pub fn path(&self, path_index: usize) -> String {
        self.paths[path_index % self.paths.len()].clone()
    }
}

pub enum RenderableKind {
    Static,
    Animated,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Wall {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Player {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Box {
    pub color: BoxColor,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct BoxSpot {
    pub color: BoxColor,
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Movable;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Immovable;

#[derive(PartialEq)]
pub enum BoxColor {
    Red,
    Blue,
}

impl Display for BoxColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BoxColor::Red => "red",
                BoxColor::Blue => "blue",
            }
        )
    }
}

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
    world.register::<Movable>();
    world.register::<Immovable>();
}
