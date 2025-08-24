fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    match number < 5 {
        true => {
            println!("match was true")
        },
        false => {
            println!("match was false")
        },
    };

    let other_number = if number % 2 == 0 { number } else {number * 2};
    println!("other number is {other_number}");

    let mut counter = 0;
    let loop_num = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("loop number is {loop_num}");

    loop_labels();
    while_loop();
    for_loop();
    countdown();
}

fn loop_labels() {
    println!();
    let mut count = 0;
    'outer: loop {
        println!("count: {count}");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'outer;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("end count: {count}");
}

fn while_loop() {
    println!();
    let mut count = 0;
    while count < 3 {
        println!("while loop count: {count}");
        count += 1;
    }
}

fn for_loop() {
    println!();
    let arr = [1, 2, 3, 54, 6];
    for num in arr {
        println!("in array: {num}");
    }
}

fn countdown() {
    println!();
    for num in (1..4).rev() {
        println!("{num}...");
    }
    println!("blast off!");
}

