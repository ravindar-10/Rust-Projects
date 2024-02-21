
mod a{
    pub fn hi(){println!("I am Hi")}
}
mod b {
    
    pub fn bye(){
        super::a::hi();//Use of super keyword to call a function inside the another module
        println!("I'm Bye")
    }
}
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
    //super keyword

}

fn main(){
    println!("hello world");
}
