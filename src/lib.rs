// this function returns the sum of two numbers
pub fn add_two(a: i32, b: i32) -> i32 {
    a + b
}

// this function returns the difference of two numbers
pub fn subtract_two(a: i32, b: i32) -> i32 {
    a - b
}

// this function returns the product of two numbers
pub fn multiply_two(a: i32, b: i32) -> i32 {
    a * b
}

// this function returns the quotient of two numbers
pub fn divide_two(a: i32, b: i32) -> i32 {
    a / b
}

// this function takes two numbers and a character like '+' or '-' or '*' or '/' and returns the corresponding function's result
pub fn calculate(a: i32, b: i32, operator: char) -> i32 {
    match operator {
        '+' => add_two(a, b),
        '-' => subtract_two(a, b),
        '*' => multiply_two(a, b),
        '/' => divide_two(a, b),
        _ => panic!("Invalid operator"),
    }
}
