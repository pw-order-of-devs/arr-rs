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

impl <N: Clone + std::fmt::Debug> VecInsertAt<N> for Vec<N> {

    fn insert_at(&mut self, index: usize, value: N) -> Self {
        self.insert(index, value);
        self.clone()
    }
}

pub(crate) trait VecUpdateAt<N> {

    fn update_at(&mut self, index: usize, value: N) -> Self;
}

impl <N: Clone> VecUpdateAt<N> for Vec<N> {

    fn update_at(&mut self, index: usize, value: N) -> Self {
        self[index] = value;
        self.clone()
    }
}

pub(crate) trait VecReverse<N> {

    fn reverse_ext(&mut self) -> Self;
}

impl <N: Clone> VecReverse<N> for Vec<N> {

    fn reverse_ext(&mut self) -> Self {
        self.reverse();
        self.clone()
    }
}
