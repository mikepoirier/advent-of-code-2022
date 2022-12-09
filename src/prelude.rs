pub use crate::utils::input::read_input;
pub use crate::utils::input::Input;

pub type Result<T> = core::result::Result<T, anyhow::Error>;
pub type CoreResult<T, E> = core::result::Result<T, E>;

