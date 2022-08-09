use core::ptr;

/// A wrapper type around a volatile variable, which allows for volatile reads and writes
/// to the contained value. The stored type needs to be `Copy`, as volatile reads and writes
/// take and return copies of the value.
#[derive(Debug, Default)]
#[repr(transparent)]
pub struct Volatile<T: Copy>(T);

impl<T: Copy> Volatile<T> {
    /// Construct a new volatile instance wrapping the given value.
    pub fn new(value: T) -> Volatile<T> {
        Volatile(value)
    }

    /// Performs a volatile write, setting the contained value to the given value `value`.
    pub fn write(&mut self, src: T) {
        unsafe {
            // UNSAFE: Safe, we know that the internal value is valid/exists
            ptr::write_volatile(&mut self.0, src)
        }
    }

    /// Performs a volatile read of the contained value, returning a copy
    /// of the read value.
    pub fn read(&self) -> T {
        unsafe {
            // UNSAFE: Safe, we know that the internal value is valid/exists
            ptr::read_volatile(&self.0)
        }
    }

    /// Performs a volatile read of the contained value, passes a mutable reference to it to the
    /// function `f`, and then performs a volatile write of the (potentially updated) value back to
    /// the contained value.
    pub fn update<F>(&mut self, f: F)
    where
        F: FnOnce(&mut T),
    {
        let mut value = self.read();
        f(&mut value);

        self.write(value);
    }
}

impl<T: Copy> Clone for Volatile<T> {
    fn clone(&self) -> Self {
        Volatile(self.read())
    }
}
