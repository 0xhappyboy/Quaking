use crate::crypto::ports::port::Port;

/// aster
pub struct Aster {}

impl Aster {
    pub async fn new() -> Self {
        Self {}
    }
    fn get_token_price(&self) {
        todo!()
    }
}

impl Port for Aster {
    fn get_price(&self) {
        todo!()
    }

    fn buy(&self) {
        todo!()
    }

    fn sell(&self) {
        todo!()
    }
}
