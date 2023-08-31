pub(crate) trait TupleSwap {

    fn swap(self) -> Self;
}

impl <T> TupleSwap for (T, T) {

    fn swap(self) -> Self {
        (self.1, self.0)
    }
}
