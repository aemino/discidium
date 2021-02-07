use crate::events::{EventDelegate, PayloadDuplex};

pub struct RunOptions<'run, D> {
    pub(crate) delegate: &'run D,
    pub(crate) payload_duplexes: Vec<Box<dyn PayloadDuplex>>,
}

impl<'run, D: EventDelegate> RunOptions<'run, D> {
    pub fn with_delegate(delegate: &'run D) -> Self {
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
