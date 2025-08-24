fn main() {
    println!("Hello, world!");
    another_function(5);
    assignment_function();

    let val = returning_value_function(21);
    println!("func returned value: {val}");
}

fn another_function(x: i32) {
    println!("entered another function");
    println!("the value of x is {x}");
}

fn assignment_function() {
    let y = {
        let x = 3;
        // no semicolon in following line bc it is an expression and not a statement
        x + 1
    };
    println!("the value of y is {y}");
}

fn returning_value_function(a: u32) -> i32 {
    ((a + 5) as i32).try_into().expect("failed to convert u32 to i32")
}
