use anchor_lang::prelude::*;

#[error_code]
pub enum MyErrors {
    #[msg("Invalid Time Range")]
     InvalidTimeRange,
    #[msg("Already initialized to buy")]
     AlreadyInitialized,
     #[msg("You can't perform this action")]
     UnAuthorised,
     #[msg("Pool already started you cant delete now")]
     AlreadyStarted,
     #[msg("Pool Already started")]
     PoolStarted,
     #[msg("He's already whitelsuted")]
     AlreadyWhitelisted,
     #[msg("He's not whitelisted")]
     NotWhitelisted,
     #[msg("Cant leave whitelist once the pool has started")]
     CantLeaveWhiteList,
     #[msg("Not valid pool for this user")]
     InvalidPoolUser
}
