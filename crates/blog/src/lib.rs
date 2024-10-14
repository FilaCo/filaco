mod app;
mod domain;
mod infra;

pub mod prelude {
    pub mod v1 {
        pub use crate::app::*;
    }
}
