fn main() {
    // variables are immutable by default
    // but can be set to mutable by using the 
    // `mut` keyword.
    let mut x = 5;
    println!("The value of x is: {}", x);
    x += 1;
    println!("The value of x is: {}", x);

    // constants are always immutable and 
    // general style is to name constants with
    // all uppercase. constants must be assigned literals
    // and cannot be assigned the return value of a function.
    // NOTE: you can put underscores in int literals to 
    // improve readibility.
    const MAX_SCORE: u32 = 100_000;
    println!("The value of my constant is {}", MAX_SCORE);

    // Shadowing:

    // you can shadow a declared variable by using the `let` keyword again
    // for example:
    let y = 3;
    // shadow y to y^2:
    let y = y * y;
    println!("The value of y is: {}", y);

    // shadowing allows us to make a few transformations of a variable
    // without making it mutable downstream. Essentially we are creating
    // a new variable with each `let` statement. This also allows us to 
    // change types of a variable:
    let spaces = "  ";
    let _spaces = spaces.len();

    // NOTE: we cannot change a mutable variables type. 
    // This is not allowed:
    // let mut spaces = "   ";
    // spaces = spaces.len();

    //////////////////////////

    // Data Types:

    // Scalars - four types: ints, floats, bools, and characters
    // Integers: 
    // u and i declare whether an int is unsigned or signed respectively.
    // signed integers allow for negative values. Rust defaults integers
    // to i32 (signed 32-bit integers)

    // Floating Point numbers
    // floating point numbers have two types f32 and f64.
    let _x = 2.0; // default float type is f64
    let _y: f32 = 2.0; // f32 float type

    // Numeric Operations:
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    // Booleans
    let t = true;
    let f: bool = false; //explicit

    // Characters:
    // four bytes for characters (more than just ASCII)
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    //////////////////////////////
    
    // Compound types: tuples and arrays

    // Tuples:
    // fixed len size, once declared, size is immutable.
    let tup: (i32, f64, u8) = (500, 6.4, 1); // explicit types
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    // indexing tuples
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    //Arrays:
    // All types of an array must be same type.
    // also fixed len size once declared. Vectors allow for 
    // variable lengths.
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // declare array with i32 types and 5 elements
    let a = [3; 5]; // evaluates to [3, 3, 3, 3, 3]

    // array access
    let first = a[0];
    
    // Functions
    secondary_function(5);
    // statements vs. expressions
    // statements do not return anything while expressions do.
    // cannot do let x = (let y = 5) (equivalent to x = y = 5 in other languages)
    // this is because `let` is a statement and does not return anything.
    // when writing a function that you want to be an expression (return some value)
    // then the return value must not be followed by a semi-colon.
    // if you end with a semi-colon you turn that function into a statement.
    expression_v_statement();

    let x = five();
    println!("Value of x returned from function: {}", x);

    // Control Flow:
    let x: i32 = 3;
    control_flow(3);
    let y: i32 = 5;
    control_flow(y);

    // part of a `let` expression
    let condition = false;
    let number = if condition { 5 } else { 6 }; // both arms must be of the same type

    println!("The value of number is: {}", number);

    // Loops - `loop`, `while`, and `for`
    // loop - repeat code forever or until explicit stop (i.e. break)
    loop_function();

    // while loops - conditional exiting
    while_function();

    // for loops - loop through a collection
    for_function();
}

// functions
fn secondary_function(x: i32){
    println!("Accessed secondary function! Argument value is {}", x)
}

fn expression_v_statement(){
    // statement
    let x = 5;
    println!("Print statement value x: {}", x);
    let y = {
        // expression
        // x scope here is local, does not shadow
        // parent-scope x value.
        let x = 3;
        x + 1
    };
    println!("Print x value after expression {}", x);
    println!("Print value of y: {}", y)
}


fn five() -> i32{
    // return is implicit and will return last line
    // but can use explicit `return` keyword to return value early.
    5
}

fn control_flow(x: i32){
    if x < 5{
        println!("Condition was true!");
    } else {
        println!("Condition was false!");
    }
}

fn loop_function(){
    let mut counter = 0;
    // loop in `let` statement
    let result = loop {
        counter += 1;
        // can return value after `break`
        if counter == 10 {
            break counter * 10
        }
    };

    println!("The result of loop is {}", result);
}

fn while_function(){
    let mut counter = 0;
    while counter < 5 {
        counter += 1
    }
    println!("While loops: Went through {} loops before exiting!", counter);
}

fn for_function(){
    let a = [0, 10 , 20, 30];
    for elem in a.iter(){
        println!("For loop iterating, item: {}", elem);
    }
}