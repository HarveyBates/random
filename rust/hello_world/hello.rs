
fn main() {
    // Type explicit 
    let logical: bool = true;
    // Infered type
    let test = true;
    println!("{} {}", logical, test);


    // Mutable variable
    let mut name = "Steve";
    println!("{}", name);
    name = "John";
    println!("{}", name);


    struct Foo {
        name: String,
        age: i16
    }

    let new = Foo {
        name: "Harvey".to_string(),
        age: 26
    };

    println!("{} {}", new.name, new.age);


    println!("Hello World!");
}


