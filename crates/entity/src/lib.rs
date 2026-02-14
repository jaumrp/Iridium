mod entity_type;
pub mod types;

use ahash::AHashMap;
pub use entity_type::EntityType;
use nbt::{Compound, Value};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Transform {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
}

pub struct BaseEntity {
    pub id: i32,
    pub uuid: uuid::Uuid,
    pub transform: Transform,
    pub on_ground: bool,
    pub entity_type: EntityType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardinalDirection {
    North,
    South,
    East,
    West,
}

pub trait Entity {
    fn base(&self) -> &BaseEntity;
    fn base_mut(&mut self) -> &mut BaseEntity;
    fn tick(&mut self);
    fn type_id(&self) -> EntityType;
}

impl Transform {
    pub fn new(x: f64, y: f64, z: f64, yaw: f32, pitch: f32) -> Self {
        Self {
            x,
            y,
            z,
            yaw,
            pitch,
        }
    }
}

impl BaseEntity {
    pub fn new(id: i32, transform: Transform, entity_type: EntityType) -> Self {
        Self {
            id,
            uuid: uuid::Uuid::new_v4(),
            transform,
            on_ground: true,
            entity_type,
        }
    }
}

impl BaseEntity {
    pub fn save_to_nbt(&self) -> Compound {
        let mut nbt = AHashMap::new();

        let pos = vec![
            Value::Double(self.transform.x),
            Value::Double(self.transform.y),
            Value::Double(self.transform.z),
        ];

        let rot = vec![
            Value::Float(self.transform.yaw as f32),
            Value::Float(self.transform.pitch as f32),
        ];

        let uuid = self.uuid.as_u128();
        let uuid = vec![
            (uuid >> 96) as i32,
            (uuid >> 64) as i32,
            (uuid >> 32) as i32,
            uuid as i32,
        ];

        nbt.insert("Pos".to_string(), Value::List(pos));
        nbt.insert("Rotation".to_string(), Value::List(rot));
        nbt.insert("UUID".to_string(), Value::IntArray(uuid));

        nbt
    }

    pub fn load_from_nbt(&mut self, nbt: &Compound) {
        if let Some(Value::List(pos)) = nbt.get("Pos") {
            if pos.len() == 3 {
                if let Value::Double(v) = pos[0] {
                    self.transform.x = v;
                }
                if let Value::Double(v) = pos[1] {
                    self.transform.y = v;
                }
                if let Value::Double(v) = pos[2] {
                    self.transform.z = v;
                }
            }
        }

        if let Some(Value::List(rot)) = nbt.get("Rotation") {
            if rot.len() == 2 {
                if let Value::Float(v) = rot[0] {
                    self.transform.yaw = v;
                }
                if let Value::Float(v) = rot[1] {
                    self.transform.pitch = v;
                }
            }
        }

        if let Some(Value::IntArray(ints)) = nbt.get("UUID") {
            if ints.len() == 4 {
                let uuid = ((ints[0] as u128) << 96)
                    | ((ints[1] as u128) << 64)
                    | ((ints[2] as u128) << 32)
                    | (ints[3] as u128);
                self.uuid = uuid::Uuid::from_u128(uuid);
            }
        }
    }
}
