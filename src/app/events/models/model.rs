use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddEventDto {
    pub organization: uuid::Uuid,
    pub branch: uuid::Uuid,
    pub schedule: uuid::Uuid,
}
