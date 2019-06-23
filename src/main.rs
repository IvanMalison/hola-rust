use core::hash::Hash;
use std::collections::HashMap;

struct Cacher<A, B, T>
    where T: Fn(A) -> B
{
    cache: HashMap<A, B>,
    calculation: T,
}

impl<A, B, T> Cacher<A, B, T>
where A: Eq + Hash + Clone,
      B: Clone,
      T: Fn(A) -> B,
{
    fn new(calculation: T) -> Cacher<A, B, T> {
        Cacher {
            cache: HashMap::new(),
            calculation,
        }
    }

    fn add(&mut self, arg: A) {
        self.cache.insert(arg.clone(), (self.calculation)(arg.clone()));
    }

    fn run(&mut self, arg: A) -> B {
        match self.cache.get(&arg) {
            Some(v) => v.clone(),
            None => {
                self.add(arg.clone());
                self.run(arg)
            },
        }
    }

    fn run_ref_alt(&mut self, arg: A) -> &B {
        match self.cache.get(&arg) {
            // I think one can't just use _v below because it changes the
            // lifetime of the immutable self borrow that results from to extend
            // long enough to interfere with the mutable borrow of self that is
            // required by the call to add. It's not entirely clear to me why
            // the compiler isn't smart enough to see that we don't actually
            // need the extended lifetime for the cases where we actually do
            // mutation, but I suspect that this is the issue
            Some(_v) => self.cache.get(&arg).unwrap(), // _v,
            None => {
                self.add(arg.clone());
                self.run_ref_alt(arg)
            },
        }
    }

    fn run_ref(&mut self, arg: A) -> &B {
        if ! self.cache.contains_key(&arg) {
            self.add(arg.clone());
        }
        self.cache.get(&arg).unwrap()
    }
}

fn main() {
    let mut c = Cacher::new(|a| {
        println!("Running for {}", a);
        a
    });

    let v1 = c.run(1);
    let v2 = c.run(2);

    println!("{}", v1);
    println!("{}", v2);

    let v3 = c.run(1);
    let v4 = c.run(2);
    let v5 = c.run(3);

    println!("{}", v3);
    println!("{}", v4);
    println!("{}", v5);
    println!("{}", v5);
    println!("{}", c.run_ref(4));
    println!("{}", c.run_ref_alt(4));
    println!("{}", c.run(4));
}
