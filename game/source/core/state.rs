pub struct GameState {
    pub total_time: f32, // Total time since game start in seconds
    pub delta_time: f32, // Time since last frame in seconds
}

impl GameState {
    pub fn new() -> Self {
        Self {
            total_time: 0.0,
            delta_time: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.delta_time = delta_time;
        self.total_time += delta_time;
    }
}
