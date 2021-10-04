use crate::progress_bar::{load_dots, load_ascii_bar, load_dots_one_line};
use std::ptr::null;

mod progress_bar;

fn main() {
   // load(7, String::from("Updating"))
   // load_ascii_bar(7);
   load_dots_one_line(7, String::from("Updating"))
}
