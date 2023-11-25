pub(crate) trait Entity: PartialEq {
    type Id;

    fn id(&self) -> &Self::Id;
}
