use crate::v1::{Container, Entry, Injectable, YadiResult};

pub trait Builder {
    fn register<T: Injectable>(&mut self, entry: impl Entry<T>) -> &mut Self;
    fn register_singleton<T: Injectable>(&mut self) -> &mut Self;
    fn register_transient<T: Injectable>(&mut self) -> &mut Self;
    fn build(self) -> YadiResult<impl Container>;
}