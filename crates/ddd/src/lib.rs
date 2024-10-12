mod app;
mod domain;

pub mod prelude {
    pub mod v1 {
        pub use crate::{
            app::{Command, Query},
            domain::{AggregateRoot, Entity, Version, VO},
        };
    }
}
