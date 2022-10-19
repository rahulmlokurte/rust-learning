fn main() {
    println!("Hello, world!");
    another_function();
    let y = function_with_parameters_return_type(10);
    println!("The value of y: {:?}", y);
    println!("The value of function_with_parameters_return_type_semicolon {0}", function_with_parameters_return_type_semicolon(10));
    let expression = {
        let y1 = 6;
        y1 + 1
    };
    println!("The value of expression: {:?}", expression);
}

fn another_function() {
    println!("Hello from another function");
}

fn function_with_parameters_return_type(x: i32) -> i32 {
    x + 1
}

fn function_with_parameters_return_type_semicolon(x: i32) -> i32 {
    return x + 1;
}
