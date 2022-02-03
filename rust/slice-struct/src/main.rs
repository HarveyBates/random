
fn first_word(s: &str) -> &str { // Return string slice (&str)
    // Takes a string and returns the first word it finds
    
    let bytes = s.as_bytes(); // Convert to array of bytes

    for(i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


fn second_word(s: &str) -> &str {
    // By using &str we can use both String and slices (str)
    let bytes = s.as_bytes();
    let len = s.len();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[i+1..len];
        }
    }

    &s[..] // Return whole word
}


fn main() {
    let mut s = String::from("Hello World!");
    let word = first_word(&s);
    println!("{}", word);
    
    let s_word = second_word(&s);
    println!("{}", s_word);
    s.clear();

    // Slices
    let hw = String::from("Hello World");
    let _hello = &hw[..5]; // or [0..5]
    let len = hw.len();
    let _world = &hw[6..len]; // or [6..11]







}
