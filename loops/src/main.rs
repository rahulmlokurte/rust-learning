fn main() {
    println!("Learning Loops");
    let mut counter = 0;
    println!("Simple Loop");
    let result = loop {
      counter += 1;
        if counter == 10 {
            break counter + 5;
        };
    };
    println!("The result of counter: {:?}", counter);
    println!("The result of result: {:?}", result);

    println!("Multiple loops");

    let mut multi_counter = 0;
    'counting_up: loop {
        println!("multi_counter = {}", multi_counter);
        let mut remaining_counter = 10;
        loop {
            println!("Remaining counter: {}", remaining_counter);
            if remaining_counter == 9 {
            break;
            }
            if multi_counter == 2 {
                break 'counting_up;
            }
            remaining_counter -= 1;
        }
        multi_counter += 1;
    }
println!("End multi_counter = {}", multi_counter);

    println!("Learning while");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }

    println!("Learning for loop");
    for element in a {
        println!("the value using for loop is: {}", element);
    }

    for number in (1..4).rev() {
        println!("The value of enhanced for loop is: {}", number);
    }

    for numbers in a.iter() {
        println!("The iteration using array.iter() function: {}", numbers);
    }
}
