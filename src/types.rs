#[derive(Debug, PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug)]
pub struct Recipe {
    pub id: String,
    pub name: String,
    pub rating: f32,
    pub difficulty: Difficulty,
    pub active_mins: u32,
    pub total_mins: u32,
}

impl Default for Recipe {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            rating: Default::default(),
            difficulty: Difficulty::Medium,
            active_mins: Default::default(),
            total_mins: Default::default(),
        }
    }
}
