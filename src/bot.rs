use catscope_bot_guest;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::extra::key_generate;

// Global variable
//pub static mut GLOBAL: MyBot = MyBot::new();
pub static mut GLOBAL: usize = 0;

pub struct Wrapper {}

pub struct Bot {
    key: Keypair,
}

impl Bot {
    pub(crate) fn new() -> Self {
        let key = key_generate().unwrap();
        Self { key }
    }
    pub(crate) fn init(&self) {
        eprintln!("turning bot on; key {}", self.key.pubkey());
    }
    pub(crate) fn shutdown(&self) {
        eprintln!("shutting down bot");
    }

    fn slot(&mut self, slot: u64, status: u8) -> () {
        eprint!("on slot: {} {}", slot, status);
    }

    fn transactionv1(&mut self, signature: Vec<u8>, slot: u64, status: u8, txresult: u32) -> () {
        eprint!("tx v1 {}", slot);
    }

    fn accountv1(
        &mut self,
        id: u64,
        slot: u64,
        lamports: u64,
        rent: u64,
        owner: u64,
        datasize: u32,
    ) -> () {
        eprint!("account v1 {}", id);
    }

    fn tokenv1(
        &mut self,
        id: u64,
        slot: u64,
        lamports: u64,
        mint: u64,
        owner: u64,
        balance: u64,
    ) -> () {
        eprint!("token v1 id {}", id);
    }
}

impl catscope_bot_guest::catscopevalidator::exports::catscope::witbot::updater::Guest for Wrapper {
    fn slot(slot: u64, status: u8) -> () {
        let ptr = unsafe { GLOBAL as *mut Bot };
        let bot: &mut Bot = unsafe { &mut *ptr };
        bot.slot(slot, status);
    }

    fn transactionv1(signature: Vec<u8>, slot: u64, status: u8, txresult: u32) -> () {
        let ptr = unsafe { GLOBAL as *mut Bot };
        let bot: &mut Bot = unsafe { &mut *ptr };
        bot.transactionv1(signature, slot, status, txresult);
    }

    fn accountv1(id: u64, slot: u64, lamports: u64, rent: u64, owner: u64, datasize: u32) -> () {
        let ptr = unsafe { GLOBAL as *mut Bot };
        let bot: &mut Bot = unsafe { &mut *ptr };
        bot.accountv1(id, slot, lamports, rent, owner, datasize);
    }

    fn tokenv1(id: u64, slot: u64, lamports: u64, mint: u64, owner: u64, balance: u64) -> () {
        let ptr = unsafe { GLOBAL as *mut Bot };
        let bot: &mut Bot = unsafe { &mut *ptr };
        bot.tokenv1(id, slot, lamports, mint, owner, balance);
    }
}
