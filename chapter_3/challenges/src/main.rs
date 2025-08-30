use std::cmp::Ordering;

macro_rules! f_to_c {
    ($f:expr) => {
        {
            (($f - 32) * 5) / 9
        }
    }
}

macro_rules! c_to_f {
    ($c:expr) => {
        {
            ($c * 9) / 5 + 32
        }
    }
}

fn test_celcius_farenheit_conversion() {
    struct CelciusFarenheitTest {
        celcius: i32,
        farenheit: i32,
    }

    let test_cases: [CelciusFarenheitTest; 3] = [
       CelciusFarenheitTest { celcius: 32, farenheit: 90 }, 
       CelciusFarenheitTest { celcius: 43, farenheit: 110 },
       CelciusFarenheitTest { celcius: -7, farenheit: 20},
    ];

    for test in test_cases {
        println!("celcius: {0} - farenheit: {1}", test.celcius, test.farenheit);
        let calculated_celcius = f_to_c!(test.farenheit);
        if calculated_celcius < test.celcius - 1 || calculated_celcius > test.celcius + 1 {
            println!("CELCIUS FAILED");
        } else {
            println!("CELCIUS PASSED");
        }
        
        let calculated_farenheit = c_to_f!(test.celcius);
        if calculated_farenheit < test.farenheit - 1 || calculated_farenheit > test.farenheit + 1 {
            println!("FARENHEIT FAILED");
        } else {
            println!("FARENHEIT PASSED");
        }
        println!();
    }
}

fn fib_dp(num: u32) -> u32 {
    match num {
        0 => 0,
        1 => 1,
        _ => { 
            let (mut a, mut b) = (0 ,1);
            for _ in 2..num {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}

fn fib_recurse(num: u32) -> u32 {
    match num {
        0 => 0,
        1 => 1,
        _ => fib_recurse(num - 1) + fib_recurse(num - 2),
    }
}

fn test_fib_function(fib_fn: fn(u32) -> u32) {
    println!("testing fib function...");

    struct FibTestCase {
        fib_num: u32,
        value: u32,
    }

    let test_cases: [FibTestCase; 9] = [
       FibTestCase { fib_num: 0, value: 0 }, 
       FibTestCase { fib_num: 1, value: 1 },
       FibTestCase { fib_num: 2, value: 1 }, 
       FibTestCase { fib_num: 3, value: 2 }, 
       FibTestCase { fib_num: 4, value: 3 },
       FibTestCase { fib_num: 5, value: 5 }, 
       FibTestCase { fib_num: 6, value: 8 }, 
       FibTestCase { fib_num: 7, value: 13 },
       FibTestCase { fib_num: 8, value: 21 },
    ];

    for test in test_cases {
        let actual = fib_fn(test.fib_num);
        match actual.cmp(&test.value) {
            Ordering::Equal => println!("PASSED"),
            _ => {
                println!("FAILED");
                break;
            }
        }
    }
}

fn main() {
    test_celcius_farenheit_conversion();
    test_fib_function(fib_dp);
    test_fib_function(fib_recurse);
}

