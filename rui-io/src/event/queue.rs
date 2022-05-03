use std::marker::PhantomData;

pub trait Enqueue<T> {
    fn enqueue(&mut self, x: T);
}

pub trait Dequeue<T> {
    fn dequeue(&mut self) -> Option<T>;
}

pub trait Queue<T>: Enqueue<T> + Dequeue<T> {}

impl<'a, T> dyn Queue<T>
where
    Self: 'a,
{
    pub fn as_iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            queue: self,
            _t: PhantomData,
        }
    }
}

pub struct IterMut<'a, T> {
    queue: &'a mut dyn Queue<T>,
    _t: PhantomData<T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.dequeue()
    }
}
