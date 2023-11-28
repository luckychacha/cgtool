use crate::error;

pub struct EthPrivateKey {
    inner: String,
}

impl EthPrivateKey {
    pub fn new(inner: String) -> Self {
        Self { inner }
    }
    pub fn generate(&self) -> Result<(), error::CgtoolError> {
        todo!()
    }
}
