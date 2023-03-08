use crate::*;

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 360.0;

pub struct Game {
    pub players: HashMap<u32, proto_all::Entity>,
    pub bodies: HashMap<u32, proto_all::Body>, // Stored as just a vector in the tutorial. But thats cring.
}
impl Game {
    pub fn new() -> Self {
        Game {
            players: HashMap::new(),
            bodies: HashMap::new(),
        }
    }
    pub fn init(&mut self) {
        self.add_cuboid(0.0, WINDOW_HEIGHT - 10.0, WINDOW_WIDTH, 10.0, 0.0);
    }
    pub fn update(&mut self) {
        
    }
    pub fn set_input(&mut self, id: u32, input: proto_all::GameInput) {
        let mut player = self.players.get_mut(&id).unwrap();
        player.x = input.x;
        player.y = input.y;
        player.pressed = input.pressed;
    }
}