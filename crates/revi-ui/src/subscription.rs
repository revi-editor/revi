use super::event::Event;

pub type Command<T> = Box<dyn Fn(Event) -> Option<T>>;

pub struct Subscription<T>(pub Vec<Command<T>>);

impl<T> Subscription<T> {
    pub fn none() -> Self {
        Self(vec![])
    }

    pub fn push(mut self, func: impl Fn(Event) -> Option<T> + 'static) -> Self {
        self.0.push(Box::new(func));
        self
    }
}
