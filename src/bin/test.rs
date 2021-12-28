use std::collections::{HashMap, HashSet};

fn main() {
    let mut a = HashMap::new();
    a.insert(1,1);
    a.insert(2,2);
    a.insert(3,3);

    a = a.into_iter().map(|(x,y)| (x,2*y)).collect();
    println!("{:?}", a)
}
