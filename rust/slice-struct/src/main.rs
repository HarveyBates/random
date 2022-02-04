
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


struct User {
    active: bool,
    username: String, 
    email: String,
    sign_in_count: u64,
}


fn build_user(email: String, username: String) -> User {
    User {
        email, 
        username, 
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)] // Enables the use of {:?}
struct Rectangle {
    width: u32,
    height: u32,
}

// One way of doing this
fn area(rec: &Rectangle) -> u32 {
    rec.width * rec.height
}

// Proper way
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
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


    let mut user: User = build_user("hbates@hotmail.com".to_string(), "Hello".to_string());

    println!("Email: {}", user.email);

    // Update user syntax
    user = User {
        email: String::from("newEmail@hotmail.com"),
        ..user // Use rest of values from existing user
    };

    println!("Email: {}", user.email);

    let rec = Rectangle {
        width: 30,
        height: 50,
    };

    dbg!(&rec); // Cool debug output with line number and filename

    println!("Area = {}", area(&rec)); // One method

    println!("Area = {}", rec.area()); // Correct method
}
