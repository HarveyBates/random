fn main() {

    // Type must be defined
    let guess: i32 = "32".parse().expect("Not a number");
    println!("{}", guess);

    // === Tuple ===
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Destructure tuple
    let (x, y, z) = tup;
    println!("x:{} y:{} z:{}", x, y, z);

    // Alternative access tuple values
    println!("x:{} y:{} z:{}", tup.0, tup.1, tup.2);

    // === Array ===
    let _arr = [1, 2, 3, 4, 5];
    // or
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    for item in arr{
        println!("{}", item);
    }

    let _sameArr = [3; 5]; // Equals [3, 3, 3, 3, 3]

    // Indexing array outside of size will panic not return
    // e.g. _sameArr[6] // Runtime error!
}
