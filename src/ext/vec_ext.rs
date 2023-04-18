pub(crate) trait VecRemoveAt<N> {

    fn remove_at(&mut self, index: usize) -> Self;
}

impl <N: Clone> VecRemoveAt<N> for Vec<N> {

    fn remove_at(&mut self, index: usize) -> Self {
        self.remove(index);
        self.clone()
    }
}

pub(crate) trait VecInsertAt<N> {

    fn insert_at(&mut self, index: usize, value: N) -> Self;
}

impl <N: Clone> VecInsertAt<N> for Vec<N> {

    fn insert_at(&mut self, index: usize, value: N) -> Self {
        self.insert(index, value);
        self.clone()
    }
}