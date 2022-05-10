use crate::scheduler::task::task::Task;
use crate::scheduler::task::waker::RawWaker;
use crate::scheduler::task::Status;
use std::future::Future;
use std::ptr::NonNull;
use std::task::Context;

pub struct VTable {
    poll: unsafe fn(NonNull<()>, cx: &mut Context<'_>) -> Status,
    drop: unsafe fn(NonNull<()>),
}

/// The lifetime 'scheduler ensures that the RawTask doesn't
/// outlive the borrowed data of the future that is inside the raw task
pub struct RawTask {
    task: NonNull<()>,
    vtable: VTable,
    waker: RawWaker, //<'scheduler>
}

// TODO make safe with lifetime argument
impl RawTask {
    pub unsafe fn new_unchecked<F>(task: Task<F>) -> Self
    where
        F: Future, //+ 'scheduler
    {
        let task = NonNull::new_unchecked(Box::into_raw(Box::new(task)));
        RawTask {
            task: task.cast(),
            vtable: VTable {
                poll: Self::_poll::<F>,
                drop: Self::_drop::<F>,
            },
            waker: RawWaker::new_unchecked(task),
        }
    }

    // The polling thread must ensure &mut self
    pub fn poll(&mut self) -> Status {
        let mut cx = Context::from_waker(self.waker.waker());
        unsafe { (self.vtable.poll)(self.task, &mut cx) }
    }

    unsafe fn _poll<F>(task: NonNull<()>, cx: &mut Context<'_>) -> Status
    where
        F: Future, //+ 'scheduler
    {
        let task = task.cast::<Task<F>>().as_mut();
        task.poll(cx)
    }

    unsafe fn _drop<F>(task: NonNull<()>)
    where
        F: Future, //+ 'scheduler
    {
        // Drop task
        Box::from_raw(task.cast::<Task<F>>().as_ptr());
    }
}
impl Drop for RawTask {
    fn drop(&mut self) {
        unsafe { (self.vtable.drop)(self.task) }
    }
}
