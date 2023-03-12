use crate::*;

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 360.0;

pub struct Game {
    pub players: HashMap<u32, proto_all::Player>,
    pub bodies: HashMap<u32, proto_all::Body>, // Stored as just a vector in the tutorial. But thats cring.
    pub game_state_updates: GameUpdate,
}

impl Game {
    pub fn new() -> Self {
        Game {
            players: HashMap::new(),
            bodies: HashMap::new(),
            game_state_updates: GameUpdate::new(),
        }
    }
    pub fn init(&mut self) {
        self.add_cuboid(0.0, WINDOW_HEIGHT - 10.0, WINDOW_WIDTH, 10.0, 0.0);
    }
    pub fn update(&mut self) {}
    pub fn set_input(&mut self, id: u32, input: proto_all::ClientInput) {
        let mut player = self.players.get_mut(&id).unwrap();
        player.x = input.x;
        player.y = input.y;
        player.pressed = input.pressed;

        // VERY UGLY CODE.
        // let player_update = match self.game_state_updates.players.get_mut(&id).unwrap() {
        //     Ok(p) => p,
        //     Err(err) => {
        //         self.game_state_updates.players.insert(id, player);
        //     };
        // };
        self.game_state_updates.players.insert(id, player.clone());
    }
}
// Stores changes in state. Resets after it has been broadcasted to clients.
pub struct GameUpdate {
    pub players: HashMap<u32, proto_all::Player>,
    pub bodies: HashMap<u32, proto_all::Body>, // Stored as just a vector in the tutorial. But thats cring.
}
impl GameUpdate {
    pub fn new() -> Self {
        GameUpdate {
            players: HashMap::new(),
            bodies: HashMap::new(),
        }
    }
    pub fn reset(&mut self) {
        self.players = HashMap::new();
        self.bodies = HashMap::new();
    }
}