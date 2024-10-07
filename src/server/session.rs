use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub struct Session {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub data: HashMap<String, String>,
}

impl Session {
    pub fn new() -> Session {
        Session {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            data: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
        self.updated_at = Utc::now();
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }

    pub fn copy(&self) -> Session {
        Session {
            id: self.id.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            data: self.data.clone(),
        }
    }
}