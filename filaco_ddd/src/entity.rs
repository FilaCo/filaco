use crate::v1::VO;

pub trait Entity: PartialEq {
    type Id: VO;

    fn id(&self) -> &Self::Id;
}
