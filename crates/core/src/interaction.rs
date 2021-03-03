use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum InteractionType {
    Interact,
    Attack,
    InteractAt,
}