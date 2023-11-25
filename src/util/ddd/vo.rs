pub(crate) trait VO<T>: Clone + PartialEq + TryFrom<T> {
    type ValueError;

    fn value(&self) -> &T;

    fn validate(value: &T) -> Result<(), Self::ValueError>;
}
