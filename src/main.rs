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
                self.cache.insert(arg.clone(), (self.calculation)(arg.clone()));
                self.cache.get(key).unwrap().clone()
            },
        }
    }
}

fn main() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.run(1);
    let v2 = c.run(2);

    println!("{}", v1);
    println!("{}", v2);
}
