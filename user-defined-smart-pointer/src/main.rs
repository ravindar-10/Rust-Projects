use core::fmt;
use std::{
    fmt::Formatter,
    ops::{Deref, DerefMut, Drop},
};
struct MyBox<T> {
    value: T,
}

impl<T> MyBox<T> {
    pub fn new(value: T) -> Self {
        MyBox { value }
    }
}

// Allow &MyBox<T> to behave like &T
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

// Allow &mut MyBox<T> to behave like &mut T
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
// Custom Drop logic
impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox containing {:?}", std::any::type_name::<T>());
    }
}

// Implement Debug properly
impl<T: fmt::Debug> fmt::Debug for MyBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("MyBox").field(&self.value).finish()
    }
}

fn main() {
    let mut x: MyBox<String> = MyBox::new(String::from("Hello"));
    // Deref coercion:  automatically calls deref()
    println!("{}", x.len());
    // Derefmut coercion
    x.push_str("world ");
    println!("{:?}", x);
}
