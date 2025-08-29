use catscope_bot_guest::catscopevalidator::catscope::witbot::{self, transactionprocessor};
use solana_sdk::clock::Slot;
use solana_sdk::signature::Keypair;

use crate::err::TinyBotError;

pub(crate) fn key_generate() -> Result<Keypair, TinyBotError> {
    let d = transactionprocessor::keygen()?;
    assert!(32 <= d.len());
    let subbuf = &d[0..32];
    Ok(Keypair::new_from_array(subbuf.try_into().unwrap()))
}

/// Send a transaction out.
pub(crate) async fn send(signature: &[u8], transaction: &[u8]) -> Result<Slot, TinyBotError> {
    let fr = transactionprocessor::send(signature, transaction);
    let slot = fr.await?;
    Ok(slot)
}
