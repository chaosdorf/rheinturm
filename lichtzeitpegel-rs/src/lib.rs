pub struct LichtZeitPegel {
    bits: u64,
}

impl LichtZeitPegel {
    fn new() -> Self {
        Self { bits: 0 }
    }

    fn set(&mut self, index: u8, value: bool) {
        assert!(index < 50);
        let mask = 1u64 << index;
        if value {
            self.bits |= mask;
        } else {
            self.bits &= !mask;
        }
    }

    fn get(&self, index: u8) -> bool {
        assert!(index < 50);
        (self.bits >> index) & 1 == 1
    }

    fn from_timestamp(time: chrono::NaiveTime) -> Self {}
}
