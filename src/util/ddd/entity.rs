use crate::util::ddd::VO;

pub(crate) trait Entity {
    type Id: VO + PartialEq;

    fn id(&self) -> &Self::Id;
}