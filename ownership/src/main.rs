fn main() {
    println!("Learning ownership");
    let s = String::from("hello");
    println!("The string from string literal is: {}",s);

    let mut s_mut = String::from("Hello ");
    s_mut.push_str("Mutable");
    println!("The string from string literal is: {}",s_mut);
    integer_binding();
    stack_string_binding();
    heap_string_binding();
}

fn heap_string_binding() {
    let s1 = String::from("Hello");
    println!("The string from string literal is: {}",s1);
    let s2 = s1;
   // println!("The string from string literal is: {}",s1);
    println!("The string s2: {}",s2);
}

fn stack_string_binding() {
    let s1 = "hello";
    let s2 = s1;
    println!("The string s2 is: {}",s2);
    println!("The string s1 is: {}",s1)
}

fn integer_binding() {
   let x = 5;
    let y = x;
    println!("The variable y is: {}",y);
}
