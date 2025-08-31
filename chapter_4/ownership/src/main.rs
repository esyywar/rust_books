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
}
