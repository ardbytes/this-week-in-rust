use std::slice::Iter;

struct EndlessIterator<'a, T> {
    orig: Iter<'a, T>,
    current: Iter<'a, T>,
}

impl<'a, T> EndlessIterator<'a, T> {
    fn new(iter: Iter<'a, T>) -> Self {
        let cloned_iter = iter.clone();
        EndlessIterator {
            orig: iter,
            current: cloned_iter,
        }
    }
}

impl<'a, T> Iterator for EndlessIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.current.next() {
            None => {
                self.current = self.orig.clone();
                self.current.next()
            },
            n => n,
        }
    }
}

fn main() {
    let v = vec![1, 2, 3, 4];
    let endless_iterator = EndlessIterator::new(v.iter());
    for i in endless_iterator {
        dbg!(i);
    }
}
