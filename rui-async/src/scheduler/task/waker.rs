use crate::scheduler::task::task::Task;
use std::future::Future;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::task::RawWakerVTable;

pub struct RawWaker {
    waker: std::task::Waker,
    //todo make safe with lifetime argument _lifetime: PhantomData<&'scheduler ()>
}
impl RawWaker {
    pub unsafe fn new_unchecked<F>(task: NonNull<Task<F>>) -> Self
    where
        F: Future,
    {
        RawWaker {
            waker: std::task::Waker::from_raw(Self::raw_waker::<F>(task.cast())),
            //_lifetime: PhantomData
        }
    }

    pub fn waker(&self) -> &std::task::Waker {
        &self.waker
    }

    unsafe fn clone_waker<F>(data: *const ()) -> std::task::RawWaker
    where
        F: Future, //+ 'scheduler
    {
        Self::raw_waker::<F>(NonNull::new_unchecked(data as *mut _))
    }

    unsafe fn drop_waker<F>(data: *const ())
    where
        F: Future, //+ 'scheduler
    {
        //harness.drop_reference();
    }

    unsafe fn wake_by_val<F>(data: *const ())
    where
        F: Future, //+ 'scheduler
    {

        //harness.wake_by_val();
    }

    // Wake without consuming the waker
    unsafe fn wake_by_ref<F>(data: *const ())
    where
        F: Future, //+ 'scheduler
    {
        //harness.wake_by_ref();
    }

    fn raw_waker<F>(data: NonNull<()>) -> std::task::RawWaker
    where
        F: Future, // + 'scheduler
    {
        let vtable = &RawWakerVTable::new(
            Self::clone_waker::<F>,
            Self::wake_by_val::<F>,
            Self::wake_by_ref::<F>,
            Self::drop_waker::<F>,
        );
        std::task::RawWaker::new(data.as_ptr(), vtable)
    }
}
