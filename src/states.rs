enum EventResult {
    Transition {},
    NoTransition {}
}

pub trait EventConsumer<E> {
    fn consume(event: E) -> EventResult;
}