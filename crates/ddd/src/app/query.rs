pub trait Query: Send {
    type Result: Send;
}
