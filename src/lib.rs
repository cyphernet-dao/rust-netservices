#[macro_use]
extern crate amplify;

#[cfg(feature = "re-actor")]
pub mod actors;

#[cfg(feature = "io-reactor")]
pub mod wire;

mod connection;
mod frame;
mod listener;
pub mod noise;
mod session;
pub mod socks5;
pub mod tunnel;

pub use connection::{NetConnection, ResAddr, StreamNonblocking};
pub use frame::{Frame, Marshaller};
pub use listener::NetListener;
pub use session::NetSession;
#[cfg(feature = "io-reactor")]
pub use wire::{ListenerEvent, NetAccept, NetTransport, SessionEvent};
