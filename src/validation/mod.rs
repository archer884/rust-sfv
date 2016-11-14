mod error;
mod validator;

pub use self::error::Error;
pub use self::validator::Validator;

pub type Result = ::std::result::Result<Validator, Error>;
