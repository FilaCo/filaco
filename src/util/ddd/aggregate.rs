use crate::util::ddd::{Entity, VO};

pub(crate) trait Aggregate: Entity {
    type Event: VO;
}