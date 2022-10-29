use std::os::unix::io::AsRawFd;
use streampipes::Resource;

use crate::InputEvent;

/// Resource subtype which is able to handle events from the file descriptors
pub trait FdResource: Resource + AsRawFd + Sized {
    /// Blocks on reading input event from the file descriptor resource.
    ///
    /// Returns number of read events.
    fn handle_readable(&mut self, events: &mut Vec<InputEvent<Self>>)
        -> Result<usize, Self::Error>;

    /// Blocks on reading output event from the file descriptor resource
    ///
    /// Returns number of read events.
    fn handle_writable(&mut self, events: &mut Vec<InputEvent<Self>>)
        -> Result<usize, Self::Error>;
}
