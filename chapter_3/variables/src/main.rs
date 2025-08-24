const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    let x = 5;
    print_type_of(&x);
    println!("the value of x is {x}");
    let x = 6;
    println!("the value of x is {x}");

    let x = x + 1;
    println!("the value of x is {x}");

    {
        let x = x * 2;
        println!("the value of x within inner scope is {x}");
    }

    println!("the value of x in outer scope is {x}");

    // x can even change data types 
    let x = "hello";
    println!("the value of x is {x}"); 
    print_type_of(&x);

    let x = false;
    print_type_of(&x);

    let x = 'g';
    print_type_of(&x);

    // tuple is immutable - can store values of different types
    let tup: (i32, f64, u8) = (500, 6.322, 120);
    let (x, y, z) = tup;
    println!("the value of x is {0}", tup.0);
    println!("the value of y is {y}");
    println!("the value of z is {z}");

    // array must be all same type. Stored on stack. Cannot be expanded/shrunk 
    // for dynamic size changing we need a vector type
    let arr = [1, 2, 3, 4, 5];
    println!("the value of arr first index is {0}", arr[0]);

    // can specify type and len of arr
    let arr: [i32; 5] = [1, 2, 34, 56, 78];

    // shorthand for duplicate values to fill arr
    let arr = [3; 5];

    for num in arr {
        println!("{num}");
    }
}
