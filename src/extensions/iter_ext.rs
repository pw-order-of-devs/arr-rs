use std::vec::IntoIter;

pub(crate) trait IterSorted<T>
    where Self: Sized + Clone + Iterator<Item = T> {

    fn sorted(&self) -> IntoIter<T>
        where T: Ord + Clone {
        let mut items: Vec<T> = self.clone().collect();
        items.sort();
        items.into_iter()
    }

    fn sorted_by<F>(&self, cmp: F) -> IntoIter<T>
        where F: FnMut(&T, &T) -> std::cmp::Ordering {
        let mut items: Vec<T> = self.clone().collect();
        items.sort_by(cmp);
        items.into_iter()
    }
}

impl<T, I> IterSorted<T> for I where I: Clone + Iterator<Item = T> {}
