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

fn main() {
    test_celcius_farenheit_conversion();
}

