fn takes_ownership(input_str: String) {
    println!("from inside sub-function: {input_str}");
}

fn gives_ownership() -> String {
    let s1 = String::from("string to return");
    s1
}

fn take_and_return_ownership(input_str: String) -> String {
    println!("took ownership of {input_str} and now giving back...");
    input_str
}

fn calculate_str_len(input_str: String) -> (String, u16) {
    let len = input_str.len() as u16;
    (input_str, len)
}

fn calculate_str_len_by_ref(input: &String) -> u16 {
    input.len() as u16
}

fn modify_string(input: &mut String) {
    input.push_str("- added this");
}

// cant even compile this function - compiler blocks dangling refs
// fn dangling_pointer() -> &String {
//     let s = String::from("nice string");
//     &s
// }

fn main() {
    // this is a string literal declaration
    // it is immutable and must be statically declared/assigned
    let _s = "hello";

    {
        // reset s as a String type instead of string literal
        let mut s = String::from("hello");
        println!("{s}");

        s.push_str(", world");
        println!("{s}");
        
        // s variable goes out of scope at the next curly brace
        // and heap memory allocated for s variable will be freed
        // rust automatically calle the 'drop' function to free memory
    }

    {
        let s1 = String::from("rust ownership test");
        println!("{s1}");

        let s2 = s1;
        // println!("{s1}"); this line panics!
        // panics bc rust has invalidated s1 to avoid 'double-freeing' the heap
        // memory s1 and s2 point to when they go out of scope. Instead only s2
        // is now valid and the heap memory for the string is freed when s2
        // goes out of scope
        // ie. rust does not support shallow copies - this operation is a 'move'
        
        println!("{s2}");
    }

    {
        // we can still use deep copies
        let s1 = String::from("will be deep copied");
        let mut s2 = s1.clone();
        s2.insert_str(0, "this is the copy: ");
        println!("s1={s1}; s2={s2}");
    }

    {
        let s1 = String::from("string from main function");
        takes_ownership(s1);
        // println!("main function str var: {s1}");
        // this line above causes compilation panic
        // s1 ownership is passed to the function and its memory is freed
        // when the function ends (and s1 goes out of scope)
    }

    {
        let s1 = gives_ownership();
        println!("got string: {s1}");
        let s2 = take_and_return_ownership(s1);
        // println!("string still in scope: {s1}");
        // above line panics bc s1 is now out of scope
        println!("string still in scope: {s2}");
    }

    {
        let s1 = String::from("string to get length of");
        let (s2, len) = calculate_str_len(s1);
        // this is kind of tedious... Rust has a better way to do this
        println!("string={s2}; string length: {len}");

        // the better way is to pass by reference
        // this is 'borrowing' rather than 'taking' ownership
        let _len = calculate_str_len_by_ref(&s2);
        println!("s2 still usable bc we passed by ref: {s2}");

        // we can also modify by reference
        let mut s2 = String::from("alterable string");
        modify_string(&mut s2);
        println!("{s2}");
    }

    {
        let mut s = String::from("mutable string");
        let r1 = &mut s;
        let _r2 = &mut s;
        // println!("{r1}");
        // this code panics
        // rust does not allow multiple mutable references to var to exist at same time
       
        // multiple non-mutable references are no problem
        let r1 = &s;
        let r2 = &s;
        println!("{r1} - {r2}");

        let r3 = &mut s;
        println!("{r3}");
        // println!("{r2}");
        // this becomes a problem as we cannot use mutable and non-mutable references
        // r1, r2 go out of scope when in introduce the mutable r3
    }
}
