fn main() {

    // mutability example
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    // shadowing example 1
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // shadowing example 2
    let spaces = "   ";
    let spaces = spaces.len();

    println!("The value of spaces is: {spaces}");
}
