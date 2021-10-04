use std::{thread, time};
use std::io::{self, Write};

#[inline(always)]
fn flush_stdout() -> () {
    io::stdout().flush().unwrap();
}

pub fn load_dots(limit: i32, mut txt: String) -> () {
    for i in 0..limit {
        thread::sleep(time::Duration::from_secs(1));

        println!("{}{}", txt, ".".repeat(i as usize))
    }
}

pub fn load_dots_one_line(limit: i32, mut txt: String) -> () {
    print!("{}", txt);

    for i in 0..limit {
        thread::sleep(time::Duration::from_secs(1));

        print!("{}", ".".repeat(i as usize));
        flush_stdout()
    }
}

pub fn load_ascii_bar(limit: i32) -> () {
    const BAR: &str = "█";

    for i in 0..limit {
        thread::sleep(time::Duration::from_secs(1));

        print!("{}", BAR.repeat(i as usize));
        flush_stdout()
    }
}
