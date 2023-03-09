use crate::*;

impl Game {
    pub fn add_player(&mut self, id: u32) {
        self.players.insert(
            id,
            proto_all::Entity {
                id: id,
                x: 0.0,
                y: 0.0,
                pressed: false,
                color: String::from("3898c1"),
            },
        );
    }

    pub fn remove_player(&mut self, id: u32) {
        self.players.remove(&id);
    }

    pub fn add_cuboid(&mut self, x: f32, y: f32, w: f32, h: f32, rotation: f32) {
        let id: u32 = self.bodies.len() as u32; // IDs just count.

        self.bodies.insert(id, proto_all::Body {
            id: id,
            color: String::from("1cdb2f"),
            x: x,
            y: y,
            w: w,
            h: h,
            rotation: rotation,
        });
    }
}