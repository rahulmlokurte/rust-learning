fn main() {
    let s1 = &gives_ownership();
    println!("{}", s1);

    let s2 = String::from("hello");
    println!("{}", s2);

    let s3 = takes_and_gives_back(s2);
    println!("{}", s3);

    let s = String::from("India");
    let a = &s[0..2];
    println!("The first 2 letters are: {}", a);
    let b = &s[2..s.len()];
    println!("The remaining letters are: {}", b);

    let v = vec![1,2,3];
    let total = sum(&v);
    println!("{}", total);
    println!("{:?}", v);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn sum(vector: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for item in vector {
        sum += item
    }
    sum
}
