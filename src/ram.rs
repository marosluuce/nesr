#[derive(PartialEq, Debug)]
pub struct Ram {
    memory: Vec<u8>,
}

impl Ram {
    pub fn new() -> Ram {
        Ram { memory: Vec::with_capacity(64 * 1024) }
    }
}
