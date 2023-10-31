pub(crate) trait VecRemoveAt<N> {

    fn remove_at(&mut self, index: usize) -> Self;

    fn remove_at_if(&mut self, index: usize, should_remove: bool) -> Self;
}

impl <N: Clone> VecRemoveAt<N> for Vec<N> {

    fn remove_at(&mut self, index: usize) -> Self {
        self.remove(index);
        self.clone()
    }

    fn remove_at_if(&mut self, index: usize, should_remove: bool) -> Self {
        if should_remove { self.remove(index); }
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
    fn reverse_if(&mut self, condition: bool) -> Self;
}

impl <N: Clone> VecReverse<N> for Vec<N> {

    fn reverse_ext(&mut self) -> Self {
        self.reverse();
        self.clone()
    }

    fn reverse_if(&mut self, condition: bool) -> Self {
        if condition { self.reverse() }
        self.clone()
    }
}

pub(crate) trait VecSwap<N> {

    fn swap_ext(&mut self, idx_1: usize, idx_2: usize) -> Self;
}

impl <N: Clone> VecSwap<N> for Vec<N> {

    fn swap_ext(&mut self, idx_1: usize, idx_2: usize) -> Self {
        self.swap(idx_1, idx_2);
        self.clone()
    }
}
