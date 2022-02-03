fn five() -> i32 {
    return 5; // return is optional
}

fn two_vals() -> (i32, String) {
    let s = String::from("Hello");
    let i = 5;

    (i, s) // returns here
}

fn pass_by_ref(s: &String) -> usize {
    s.len() // Note no semicolon
}

fn mut_pass_by_ref(some_str: &mut String) {
    some_str.push_str(", world!"); // Note mut in dec
}

fn main() {
    // Simple return
    println!("{}", five());

    // Return tuple
    let t = two_vals();
    println!("{} {}", t.0, t.1);

    // Return by reference
    let s1 = String::from("Hello");
    let len = pass_by_ref(&s1);
    println!("Length = {}", len);

    // Mutable by reference
    let mut s2 = String::from("Hello");
    mut_pass_by_ref(&mut s2); // Note mut is pass here too
    println!("{}", s2);

    let val = 33;
    // Inline if statement
    let new_val = if val == 33 { 5 } else { 7 };
    println!("{} {}", val, new_val);

    // For loop
    for number in (1..4).rev(){
        println!("{}!", number);
    }
    println!("LIFT OFF!");

    // Creating and editing string in the heap
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);

}

