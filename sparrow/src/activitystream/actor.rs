use serde::{Deserialize, Serialize};

pub mod person;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActorType {
    Application,
    Group,
    Organization,
    Person,
    Service,
    #[default]
    None,
}
