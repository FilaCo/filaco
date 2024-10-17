use crate::domain::aggregate_root::Blog;
use crate::domain::entity::{Attachment, Comment, Reaction};
use crate::domain::vo::post::*;
use crate::domain::vo::{attachment, comment, reaction};
use chrono::{DateTime, Local};
use ddd::prelude::v1::Version;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Eq)]
pub struct Post {
    id: Id,
    blog: Blog,
    created_at: DateTime<Local>,
    updated_at: Option<DateTime<Local>>,
    version: Version,
    attachments: HashMap<attachment::Id, Attachment>,
    comments: HashMap<comment::Id, Comment>,
    reactions: HashMap<reaction::Id, Reaction>,
}

impl Post {
    pub fn edit(&mut self) -> Result<&[PostEvent], PostError> {
        todo!()
    }
}

impl PartialEq for Post {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

#[derive(Debug, Error)]
pub enum PostError {}

#[derive(Debug)]
pub enum PostEvent {
    Edited,
}
