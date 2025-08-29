//use tokio::io::{AsyncReadExt, AsyncWriteExt};
use self::bot::{Bot, GLOBAL};
use nalgebra::DMatrix;
use std::thread::sleep;
use std::time::Duration;

pub(crate) mod bot;
pub(crate) mod err;
pub(crate) mod extra;
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Initialize it once

    let bot = Bot::new();
    unsafe {
        GLOBAL = Box::into_raw(Box::new(bot)) as usize;
    }
    let t = Duration::from_secs(10);
    let a = DMatrix::from_row_slice(2, 2, &[1.0, 2.0, 3.0, 4.0]);
    let b = DMatrix::from_row_slice(2, 2, &[5.0, 6.0, 7.0, 8.0]);
    let c = &a * &b;
    eprintln!("will sleep {:?}; matrix: {}", t, c);
    sleep(Duration::from_secs(10));

    eprintln!("slept {:?}", t);
    let bot2 = unsafe { Box::from_raw(GLOBAL as *mut Bot) };
    bot2.shutdown();
    let _ = bot2;
    //    println!("{}", String::from_utf8_lossy(&buf));
}
