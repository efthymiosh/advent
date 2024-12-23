use std::{io, io::prelude::*};
use std::time::Duration;
use std::thread::sleep;

pub fn pause() {
    io::stdin().read_exact(&mut [0]).unwrap();
}

pub fn wait_ms(milliseconds: u32) {
    sleep(Duration::new(0, milliseconds * 1000 * 1000));
}
