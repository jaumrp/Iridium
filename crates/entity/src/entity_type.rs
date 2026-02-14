#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityType {
    Player,
    Zombie,
}

impl EntityType {
    pub fn name(&self) -> String {
        match self {
            EntityType::Player => "player".to_string(),
            EntityType::Zombie => "zombie".to_string(),
        }
    }
    pub fn ident(&self) -> String {
        format!("minecraft:{}", self.name())
    }
}
