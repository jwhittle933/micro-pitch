pub trait IncrementalCounter {
    fn every(self, c: Self) -> bool;
}

impl IncrementalCounter for u64 {
    fn every(self, c: Self) -> bool {
        self % c == 0
    }
}
