use crate::crypto::ports::port::Port;


/// lighter
struct Lighter {}

impl Lighter {}

impl Port for Lighter {
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
