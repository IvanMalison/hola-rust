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

    fn run(&mut self, arg: A) -> B {
        let key = &arg;
        match self.cache.get(key) {
            Some(v) =>  v.clone(),
            None => {
                self.add(arg.clone());
                self.run(arg)
            },
        }
    }

    fn add(&mut self, arg: A) {
        self.cache.insert(arg.clone(), (self.calculation)(arg.clone()));
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
    println!("{}", c.run_ref(4));
    println!("{}", c.run(4));
}
