use core::hash::Hash;
use std::collections::HashMap;
use std::rc::Rc;

struct Cacher<A, B, T>
    where T: Fn(A) -> B
{
    cache: HashMap<A, Rc<B>>,
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
        self.cache.insert(arg.clone(), Rc::new((self.calculation)(arg.clone())));
    }

    fn run(&mut self, arg: A) -> Rc<B> {
        match self.cache.get(&arg) {
            Some(v) => Rc::clone(v),
            None => {
                self.add(arg.clone());
                self.run(arg)
            },
        }
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
    c.cache.clear();

    println!("{}", v3);
    println!("{}", c.run(3));
    println!("{}", v4);
    println!("{}", v5);
    println!("{}", v5);
    println!("{}", v1);
    println!("{}", c.run(4));
    println!("{}", c.run(4));
    println!("{}", c.run(4));
}
