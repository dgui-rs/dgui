use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::DIRTY;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Flags {
    SIGNALED,
    UNSIGNALED,
}

pub struct Signal<T> {
    pub value: Rc<RefCell<T>>,
}

impl<T> Signal<T> {
    pub fn create(value: T) -> Self {
        let data = Rc::new(RefCell::new(value));

        Self { value: data }
    }

    pub fn value(&self) -> T
    where
        T: Clone,
    {
        self.value.borrow().clone()
    }

    pub fn set(&self, f: impl FnOnce(&mut T)) {
        {
            let mut borrow = self.value.borrow_mut();
            f(&mut *borrow);
        }

        DIRTY.with(|flags| flags.set(Flags::SIGNALED));
    }
}

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self {
            value: Rc::clone(&self.value),
        }
    }
}

#[derive(Clone)]
pub enum Value<T> {
    Static(T),
    Dynamic(Signal<T>),
}

impl<T: Clone> Value<T> {
    pub fn get(&self) -> T {
        match self {
            Self::Static(value) => value.to_owned(),
            Self::Dynamic(value) => value.value(),
        }
    }
}

impl<T> From<T> for Value<T> {
    fn from(value: T) -> Self {
        Self::Static(value)
    }
}

impl From<&str> for Value<String> {
    fn from(value: &str) -> Self {
        Self::Static(value.to_string())
    }
}

impl<T: Clone> From<&Value<T>> for Value<T> {
    fn from(value: &Value<T>) -> Self {
        value.clone()
    }
}

impl<T> From<&Signal<T>> for Value<T> {
    fn from(signal: &Signal<T>) -> Self {
        Self::Dynamic(signal.clone())
    }
}

pub trait TextValue {
    fn get(&self) -> String;

    fn into_boxed(self) -> Box<dyn TextValue>;
}

impl<T: Display + Clone + 'static> TextValue for Signal<T> {
    fn get(&self) -> String {
        self.value().to_string()
    }
    fn into_boxed(self) -> Box<dyn TextValue> {
        Box::new(self)
    }
}

impl<T: Display + Clone + 'static> TextValue for &Signal<T> {
    fn get(&self) -> String {
        self.value().to_string()
    }
    fn into_boxed(self) -> Box<dyn TextValue> {
        Box::new((*self).clone())
    }
}

impl TextValue for &'static str {
    fn get(&self) -> String {
        self.to_string()
    }

    fn into_boxed(self) -> Box<dyn TextValue> {
        Box::new(self)
    }
}

impl TextValue for String {
    fn get(&self) -> String {
        self.clone()
    }

    fn into_boxed(self) -> Box<dyn TextValue> {
        Box::new(self)
    }
}
