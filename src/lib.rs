mod _bool;
mod _emote;
mod _unsafe;
mod _yeet;

/// Yes, this module does in fact do nothing. But it's working as intended.
pub mod prelude {
    pub use crate::_bool::*;
    pub use crate::_emote::*;
    pub use crate::_unsafe::*;
    pub use crate::_yeet::*;
}
