use std::vec::Vec;

pub trait DynVsImpl {
    fn bar(&self) -> &str;
}

trait MyTrait {}

struct FirstImpl;
struct SecondImpl;

impl DynVsImpl for FirstImpl {
    fn bar(&self) -> &str {
        return &"first";
    }
}

impl DynVsImpl for SecondImpl {
    fn bar(&self) -> &str {
        return &"second";
    }
}

fn foo(a: &impl DynVsImpl) {
    println!("{}", a.bar());
}

fn foo_dyn(a: &dyn DynVsImpl) {
    println!("{}", a.bar());
}


fn return_a_impl(a: i32) -> Box<dyn DynVsImpl> {
    if a > 12 {
        Box::new(FirstImpl {})
    } else {
        Box::new(SecondImpl {})
    }
}

pub fn make_vector() -> Vec<Box<DynVsImpl>> {
    let mut vec : Vec<Box<DynVsImpl>> = Vec::new();
    let first_box // : Box<DynVsImpl>
        = Box::new(FirstImpl {});
    vec.push(first_box);
    vec.push(Box::new(SecondImpl {}));
    vec
}


fn make_vector_impl() -> Vec<impl DynVsImpl> {
    let mut vec = Vec::new();
    vec.push(FirstImpl {});
    // Not allowed bc impls not the same
    // vec.push(SecondImpl {});
    vec
}

pub fn test () {
    foo(&FirstImpl {});
    foo(&SecondImpl {});
    println!("Doing with 24");
    foo_dyn(&(*return_a_impl(24)));
    println!("Doing with 0");
    foo_dyn(&(*return_a_impl(0)));
    for i in make_vector() {
        println!("{}", (*i).bar());
    }
}
