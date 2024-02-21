use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::ops::Deref;
// "Use the Box<T> type to allocate an integer on the heap.
// Implement a basic reference-counted smart pointer using Rc<T>.

// Define a basic reference-counted smart pointer struct
#[derive(Debug)]
struct MyRc<T> {
    data: Rc<T>,
}

impl<T> MyRc<T> {
    // Create a new instance of MyRc wrapping some data
    fn new(data: T) -> Self {
        MyRc {
            data: Rc::new(data),
            
        }
    }
    
}

// Implement Deref trait to allow dereferencing MyRc
impl<T> Deref for MyRc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
    
}

fn main() {
    // Create a new MyRc instance containing an integer
    let mut my_rc = MyRc::new(42);

    // Dereference and print the data contained in the MyRc instance
    println!("Data: {}", *my_rc);
      let a=Box::new(10);
    let b=a.clone();//deep copy of a
    // let c=a;//here we can not use the a again because has been already moved in line no 15
    let c=a.clone();

    //using the reference counter
    let a=Rc::new(10);
    let b=a.clone();//This creates another pointer to the same allocation, increasing the strong reference count.
    let c=a.clone();
}
