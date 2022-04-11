use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct ConcurrentQueue<T> {
    head: AtomicUsize,
    tail: AtomicUsize,
    buffer: Box<[UnsafeCell<MaybeUninit<T>>]>
}
impl<T> ConcurrentQueue<T> {

    pub fn new(size: usize) -> Self {

        let mut buffer = Vec::with_capacity(size);
        for _ in 0..size {
            buffer.push(UnsafeCell::new(MaybeUninit::uninit()));
        }

        ConcurrentQueue {
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            buffer: buffer.into_boxed_slice()
        }
    }

    pub fn try_pop(&self) -> Option<T> {
        let head = self.head.load(Ordering::Acquire);
        // Don't overtake the tail position
        if head == self.tail.load(Ordering::Acquire) {
            None
        } else {
            // Optimistically read the value.
            // Maybe this value is already uninit
            // because it got taken by another thread
            let res = unsafe { self.buffer[head % self.buffer.len()].get().read() };
            let new_head = head.wrapping_add(1);

            match self.head.compare_exchange_weak(head, new_head, Ordering::Release, Ordering::Relaxed) {
                // In this case no other thread took the value and therefore we can return it.
                // We still have to make the read operation of this value before the compare and exchange
                // because even if no other stealer thread got the value, the value could get overwritten
                // by the producer thread in the meantime
                Ok(_) => Some(unsafe {
                    res.assume_init()
                }),
                // In this case either the exchange function failed or the value did get taken by a different thread
                // because the head value was not the same anymore
                Err(_) => None
            }
        }
    }

    /**
     * Must be executed from one thread only
     */
    pub unsafe fn try_push(&self, x: T) -> Result<(), T> {
        // Because we only modifying the tail value on this thread,
        // we dont have to use the atomic load operation.
        // The store of this value still has to be performed atomically
        // because the tail value could get read from a different thread
        // in the meantime
        let tail = *(&mut *(&self.tail as *const AtomicUsize as *mut AtomicUsize)).get_mut();

        // we don't care if the head value changes in the meantime but
        // we care if we can insert safely.
        // So if the condition (tail - head) < len is true and after that the head-value changes
        // to something greater but still smaller equals tail then the condition
        // (tail - head) < len is still full-filled:
        // let head' be the changed value then following conditions are true:
        // 1. head' > head and 2. head' <= tail
        // => (tail - head') < len
        // => we can still insert safely
        let head = self.head.load(Ordering::Acquire);
        if (tail.wrapping_sub(head) as usize) < self.buffer.len() {

            self.buffer[tail % self.buffer.len()].get().as_mut().unwrap().as_mut_ptr().write(x);
            let new_tail = tail.wrapping_add(1);

            // Because we are on a single thread this operation
            // can be simply executed. We just have to make sure that the load of the tail
            // value is performed before this store. This can be achieved through memory ordering
            self.tail.store(new_tail, Ordering::Release);
            Ok(())
        } else {
            Err(x)
        }
    }
}