fn main() {
    // create a `String` type from a string literal
    // this `String` type is stored on the heap rather
    // than the stack. `::` allows us to namespace this 
    // variable without from clause (will be discussed 
    // further in future chapters)
    let s = String::from("hello");

    // allows for mutation:
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`

    // can't do this:
    // let mut s = "hello";
    // s += "world" - standard string type is immutable.
    // this is because of memory and allocation. 

    // can do this with fixed size literals because compiler knows
    // how much memory to allocate at compilation time. This decision was
    // made because copying fixed size elements on the stack is a cheap operation
    // while copying variable length elements on the heap (deep copies) are expensive:
    let x = 5;
    let y = x;

    // here since we have fixed size literals we are allocating 
    // memory on the stack. (My understanding may be a little simplistic here)
    // Therefore, we have a copy of that data (y) on the stack. However, when
    // storing data on the heap (in this case a `String` type) like so:
    let s1 = String::from("hello");
    let s2 = s1;
    // you will have two pieces of data stored for this operation (s1).
    // On the stack you will have a the size and total allowed capacity for s1
    // along with a pointer pointing to the actual data on the heap. s2 will have a similar
    // data structured stored on the stack but will point to the same data on the heap.

    // Another thing to keep in mind here is that s1 and s2 are pointing to the same memory
    // in the heap. Therefore when we try to deallocate s1 and s2 we will be trying to 
    // deallocate the same memory, leading to the `double free problem`. Rust handles this
    // internally by having this concept of `move` rather than a `shallow copy`. Essentially
    // the line `let s2 = s1;` invalidates s1, and so the following would throw a compilation
    // error: println!("{}, world!", s1); -> value borrowed here after move

    // in order to create a deep copy in rust:
    let s1 = String::from("hello");
    let mut s2 = s1.clone();
    s2.push_str(", world!");

    println!("s1 = {}, s2 = {}", s1, s2);
    

    // References and Borrowing

    // `&` special character allows you to create a reference rather than giving ownership 
    // to variables to child functions. this way the variable isn't dropped when it 
    // becomes out of scope in the child function and we can still access from this parent function:
    let s1 = String::from("hello");
    let len = reference_function(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // mutable references
    let mut s1 = String::from("hello");
    mut_reference_function(&mut s1);
    println!("Mutable s1: {}", s1);

    // this is not allowed:
    //let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // cannot reference mutable variable more than once in order to avoid data races.
    // also cannot have both a mutable and immutable reference at the same time. 

    // dangling references will not compile in rust
    // i.e. you have a variable declared in a child function and return a  reference to it,
    // since the original variable will go out of scope at the end of the child function,
    // you cannot reference it. Instead you must return the variable itself.
    

    // Slices
    let mut h = String::from("hello, world!");
    let f_word = first_word(&h);
    println!("First word is {} with original phrase {}", f_word, h);

}

fn reference_function(s: &String) -> usize{
    s.len()
}

fn mut_reference_function(s: &mut String){
    s.push_str(", world!")
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){ 
        if item == b' ' {
            return &s[..i];
        }
    }
    &s
}