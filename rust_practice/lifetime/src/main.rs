// fn largest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
//     if str1.len() > str2.len() {
//         str1
//     } else {
//         str2
//     }
// }
// struct MyIterator<'a, T> {
//     slice: &'a [T],
// }
// impl<'a, T> Iterator for MyIterator<'a, T> {
//     type Item = &'a T;
//     fn next(&mut self) -> Option<Self::Item> {
//         let (element, rest) = self.slice.split_first()?;
//         self.slice = rest;
//         Some(element)
//     }
// }
// struct MyMutableIterator<'iter, T> {
//     slice: &'iter mut [T],
// }
// impl<'iter, T> Iterator for MyMutableIterator<'iter, T> {
//     type Item = &'iter mut T;
//     fn next<>(& mut self) -> Option<Self::Item> {
//         let slice1=&mut self.slice;
//         let slice2=std::mem::replace(slice1, &mut []);
//         let (first,rest)=slice2.split_first_mut()?;
//         self.slice=rest;
//         Some(first)
        
//     }
// }
// fn main() {
//     // let r;

//     // {
//     //     let x = 5;
//     //     r = &x;
//     // }

//     // println!("r: {}", r);
//     let str_1 = "Hi I'm str1";
//     let str_2 = "Hi I'm str2";
//     println!("largest string is {}", largest(str_1, str_2));
//     let collection = vec![1, 2, 3, 4];
//     let wrapper: MyIterator<i32> = MyIterator {
//         slice: &collection[..],
//     };
//     for (index, elem) in wrapper.slice.iter().enumerate() {
//         assert_eq!(*elem, collection[index]);
//     }
//     let mut mut_collections = vec![1, 2, 3, 4];
//     let mut mut_wrapper: MyMutableIterator<i32> = MyMutableIterator {
//         slice: &mut  mut_collections[..],
//     };
//     // for (index, elem) in mut_wrapper.enumerate() {
//     //     *elem = *elem + 1;
//     //     assert_eq!(*elem, mut_collections[index] + 1);
//     // }
// }
// Lifetimes are annotated below with lines denoting the creation
// and destruction of each variable.
// `i` has the longest lifetime because its scope entirely encloses 
// both `borrow1` and `borrow2`. The duration of `borrow1` compared 
// to `borrow2` is irrelevant since they are disjoint.
fn main() {
    let i = 3; // Lifetime for `i` starts. ────────────────┐
    //                                                     │
    { //                                                   │
        let borrow1 = &i; // `borrow1` lifetime starts. ──┐│
        //                                                ││
        println!("borrow1: {}", borrow1); //              ││
    } // `borrow1` ends. ─────────────────────────────────┘│
    //                                                     │
    //                                                     │
    { //                                                   │
        let borrow2 = &i; // `borrow2` lifetime starts. ──┐│
        //                                                ││
        println!("borrow2: {}", borrow2); //              ││
    } // `borrow2` ends. ─────────────────────────────────┘│
    //                                                     │
}   // Lifetime ends. ─────────────────────────────────────┘
