use catscope_bot_guest::catscopevalidator::catscope::witbot::shooter::ErrorCode as ShooterErrorCode;
use catscope_bot_guest::catscopevalidator::catscope::witbot::transactionprocessor::ErrorCode as TxErrorCode;

#[derive(Debug)]
pub enum TinyBotError {
    Shooter(ShooterErrorCode),
    TransactionProcessor(TxErrorCode),
}

impl std::fmt::Display for TinyBotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TinyBotError::Shooter(msg) => write!(f, "shooter: {}", msg),
            TinyBotError::TransactionProcessor(msg) => {
                write!(f, "transaction processing error: {}", msg)
            }
        }
    }
}
impl From<ShooterErrorCode> for TinyBotError {
    fn from(value: ShooterErrorCode) -> Self {
        TinyBotError::Shooter(value)
    }
}
impl From<TxErrorCode> for TinyBotError {
    fn from(value: TxErrorCode) -> Self {
        TinyBotError::TransactionProcessor(value)
    }
}
