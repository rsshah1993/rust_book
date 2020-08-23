fn main() {
    // VECTORS
    // have to specify type annotation since vector is empty
    let v: Vec<i32> = Vec::new();
    // vec! macro allows us to create a vector
    // don't have to specify type annotation since we are not initializing
    // empty vector (defaults to i32)
    let v = vec![1, 2, 3];
    
    // mutable
    let mut v: Vec<i32> = vec![];
    v.push(1);
    
    ////////
    // reading elements of a vector
    let mut v = vec![1, 2, 3, 4, 5];

    // indexing syntax 
    let third: &i32 = &v[2];
    println!("Index Syntax: The third element is {}", third);

    // get syntax
    match v.get(2) {
        Some(third) => println!("Get Syntax: The third element is {}", third),
        None => println!("There is no third element"),

    }
    // Note: index syntax will panick if index is out of range while get syntax will return None

    // iteration over a vector
    // we borrow v here so that it can be used afterwards
    // otherwise it will be a borrow after move
    for i in &v {
        println!("Iteration: {}", i)
    }

    // update values in vector in place
    // * is dereferencing operator
    for i in &mut v {
        *i += 50
    }

    // by default vectors can only hold one type. However we can define an enum
    // that will allow us to hold different types: 
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Int(4),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for i in &row {
        println!("Enum iterate: {:?}", i)
    }

    // STRINGS
    // creates new empty `String`
    let mut s = String::new();

    // create string from literal
    // Note: following are all equivalent
    let data = "some string";
    let s2 = data.to_string();

    let s3 = "some string".to_string();

    let s4 = String::from("some string");

    // appending to string
    let mut s = String::from("foo");
    s.push_str("bar");

    // + operator
    let s1 = "foo".to_string();
    let s2 = "bar".to_string();

    let s = s1 + &s2;

    println!("{}", s);
    println!("{}", s2);
    // println!("{}", s1); // cannot do this since the + operator takes ownership of s1

    // format
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s1);

    // string indexing
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    // however doing &hello[0..1] would cause Rust to panick.

    println!("String Slice: {}", s);

    // string indexing is tricky, use `chars` instead: 
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

} // v gets dropped here
