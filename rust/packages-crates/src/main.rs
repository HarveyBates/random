// Packages: A Cargo feature that lets you build, test, and share crates
// Crates: A tree of modules that produces a library or executable
// Modules and use: Let you control the organization, scope, and privacy of paths
// Paths: A way of naming an item, such as a struct, function, or module

use rand::Rng;
use std::collections::HashMap;
// or
// use std::collections{HashMap, other...};
// use std::collections{self, HashMap}; // Imports colllections and collections::HashMap

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("SN: {}", secret_number);

    let mut map = HashMap::new();
    map.insert(1, 2);
    dbg!(&map);


}
