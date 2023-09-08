/// Actor messages

use super::*;

use std::{
    fmt::{Debug, Display},
    ops::Range,
};

#[derive(Message)]
#[rtype(result = "()")]
pub struct HashFound {
    pub digit: u128,
    pub hash: [u8; 32],
}

impl Debug for HashFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HashFound")
            .field("digit", &self.digit)
            .field("hash", &self.hash)
            .field("display as:", &format!("{}", &self))
            .finish()
    }
}

impl Display for HashFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: \"", self.digit).ok();
        for byte in self.hash {
            write!(f, "{:02x}", byte).ok();
        }
        write!(f, "\"").ok();

        Ok(())
    }
}

/// Manager sends this message to workers
#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct SearchInsideRange(pub Range<u128>);

// When worker is ready, it sends this message to Manager.
// Optional range param is set up if worker has finished a range.
#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct WorkerReady(pub Option<Range<u128>>);
