
use std::hash::{Hash, Hasher};

#[derive(PartialEq, Eq, Clone)]
pub struct Player {
    id: String,
    name: String,
}

impl Player {
    pub fn new(id: Option<&str>, name: Option<&str>) -> Option<Self> {
        let Some(id) = id else { return None };
        let Some(name) = name else { return None };
        // ignore placeholders
        if id == "00000000" {
            return None
        }
        let this = Self {
            id: id.into(),
            name: name.into(),
        };
        Some(this)
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

impl Hash for Player {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
