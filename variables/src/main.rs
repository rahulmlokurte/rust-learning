fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of constants is {THREE_HOURS_IN_SECONDS}");

    let y = 5;
    println!("The value of y is {y}");
    let y = 10;
    println!("The new value of y is {y}");

    let z = 5;

    let z = z + 1;

    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {z}"); //12
    }

    println!("The value of z is: {z}"); //6

    let spaces = "    ";
    let spaces = spaces.len();
    println!("The number of spaces is {spaces}");

    //Floating point Types

    let f64 = 2.0;
    let f32: f32 = 3.0;
    println!("The number of floats is f64 is {f64} and f32 is {f32}");

    // Numberic Operations

    let sum = 5 + 10;
    println!("The sum is: {}", sum);

    let differnce = 10 - 5;
    println!("The difference is: {}", differnce);

    let multiplication = 10 * 2;
    println!("The multiplication is: {}", multiplication);

    let quotient = 56.2 / 32.5;
    println!("The quotient is: {}", quotient);
    let floored = 2 / 3;
    println!("The floored is: {}", floored);
    let remainder = 2 % 3;
    println!("The remainder is: {}", remainder);

    // boolean

    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("The boolean of t: {0} and f: {1}", t, f);

    // Tuples
    let tuples: (i32, f64, u8) = (223, 3.4, 226);

    println!("The tuple of tuples: {:?}", tuples);
    println!("The tuple of tuples: {:#?}", tuples);
    println!("The first value in tuple: {}", tuples.0);
    println!("The second value in tuple: {}", tuples.1);
    println!("The third value in tuple: {}", tuples.2);

    let tupe = (400, 6.4, 3);
    let (x, y, z) = tupe;

    println!("The tupe is {:?}", tupe);
    println!("The tupe is {:#?}", tupe);
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    // Arrays

    let a = [1, 2, 3, 4, 5];
    println!("The array a is {:?}", a);
    println!("The array a is {:#?}", a);

    let b: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The array b is {:?}", b);
    println!("The array b is {:#?}", b);

    let same_number = [3; 5];
    println!("The array same_number is {:?}", same_number);
}
