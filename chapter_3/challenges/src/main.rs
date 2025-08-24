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

fn main() {
    println!("Hello, world!");
    let celcius = f_to_c!(80);
    println!("c is {celcius}");
}


