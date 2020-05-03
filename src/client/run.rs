use crate::events::{EventDelegate, PayloadDuplex};

pub struct RunOptions<'run> {
    pub(crate) delegate: &'run dyn EventDelegate,
    pub(crate) payload_duplexes: Vec<Box<dyn PayloadDuplex>>,
}

impl<'run> RunOptions<'run> {
    pub fn with_delegate(delegate: &'run dyn EventDelegate) -> Self {
        Self {
            delegate,
            payload_duplexes: Vec::new(),
        }
    }

    pub fn payload_duplexes(
        mut self,
        iter: impl IntoIterator<Item = Box<dyn PayloadDuplex>>,
    ) -> Self {
        self.payload_duplexes.extend(iter);
        self
    }
}
