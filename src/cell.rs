use std::cell::UnsafeCell;
pub struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T> {
    fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }
    pub fn set(&self, value: T) {
        unsafe { *self.value.get() = value };
    }
    pub fn get(&self) -> T {
        unsafe { *self.value.get() }
    }
}
