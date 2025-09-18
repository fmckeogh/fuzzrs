use core::fmt::{self, Write};

pub struct Pl011;

impl Pl011 {
    const ADDR: u64 = 0x0900_0000;

    fn print_char(c: char) {
        unsafe { (Self::ADDR as *mut u8).write(c as u8) };
    }

    fn print_str(str: &str) {
        str.chars().for_each(|c| Self::print_char(c));
    }
}

impl Write for Pl011 {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        Self::print_str(s);
        Ok(())
    }
}
