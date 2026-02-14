use crate::{BaseEntity, Entity, Transform};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Gamemode {
    Survival = 0,
    Creative = 1,
    Adventure = 2,
    Spectator = 3,
}

pub struct Player {
    pub base: BaseEntity,
    pub username: String,
    pub gamemode: Gamemode,
    pub flying: bool,
}

impl Player {
    pub fn new(id: i32, username: String, transform: Transform) -> Self {
        Player {
            base: BaseEntity::new(id, transform, crate::EntityType::Player),
            username,
            gamemode: Gamemode::Survival,
            flying: false,
        }
    }
}

impl Entity for Player {
    fn base(&self) -> &BaseEntity {
        &self.base
    }

    fn base_mut(&mut self) -> &mut BaseEntity {
        &mut self.base
    }

    fn tick(&mut self) {}

    fn type_id(&self) -> crate::EntityType {
        self.base.entity_type
    }
}
