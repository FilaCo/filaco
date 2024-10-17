use crate::domain::aggregate_root::Post;
use crate::domain::vo::blog::Id;
use crate::domain::vo::post;
use chrono::{DateTime, Local};
use std::collections::HashMap;

#[derive(Debug, Eq)]
pub struct Blog {
    id: Id,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
    posts: HashMap<post::Id, Post>,
}

impl Blog {
    pub fn publish_post(&mut self) {}
}

impl PartialEq for Blog {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
