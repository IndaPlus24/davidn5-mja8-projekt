
#[derive(Clone, Default)]
pub struct Block {
    pub occupied: bool,
    pub path: String,
}

impl Block {
    pub fn new() -> Self {
        Self {
            occupied: false,
            path: "test".to_string(),
        }
    }

    pub fn is_occupied(&self) -> bool {
        self.occupied
    }
}
