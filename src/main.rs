//use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::thread::sleep;
use std::time::Duration;

use self::bot::{Bot, GLOBAL};

pub(crate) mod bot;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Initialize it once
    let bot = Bot::new();
    unsafe {
        GLOBAL = Box::into_raw(Box::new(bot)) as usize;
    }
    let t = Duration::from_secs(10);
    eprintln!("will sleep {:?}", t);
    sleep(Duration::from_secs(10));

    eprintln!("slept {:?}", t);
    let bot2 = unsafe { Box::from_raw(GLOBAL as *mut Bot) };
    bot2.shutdown();
    let _ = bot2;
    //    println!("{}", String::from_utf8_lossy(&buf));
}
