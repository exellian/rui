use std::any::Any;
use std::future::Future;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::pin::Pin;
use std::task::{Context, Poll};
use crate::runtime::task::Status;
use crate::runtime::task::task::{Task};

pub struct VTable {
    poll: unsafe fn(&DynTask) -> Status,
    poll_consume: fn(&DynTask, &mut dyn Any)
}

pub struct DynTask {
    task: Box<dyn Any>,
    vtable: VTable
}

impl DynTask {

    pub fn new<F>(future: F) -> Self where F: Future + 'static {
        DynTask {
            task: Box::new(Task::new(future)),
            vtable: VTable {
                poll: Self::_poll::<F>,
                poll_consume: Self::_poll_consume::<F>
            }
        }
    }

    pub fn poll_consume<O>(&self) -> Poll<O> where O: 'static {
        let mut out: Option<Poll<O>> = None;
        (self.vtable.poll_consume)(self, &mut out);
        out.unwrap()
    }

    fn _poll_consume<F>(&self, out: &mut dyn Any) where F: Future + 'static {
        let task: &Task<F> = self.task.downcast_ref().unwrap();
        let out_mut: &mut Option<Poll<F::Output>> = out.downcast_mut().unwrap();
        *out_mut = Some(task.poll_consume());
    }

    // The polling thread must ensure &mut self
    pub unsafe fn poll(&self) -> Status {
        (self.vtable.poll)(self)
    }

    unsafe fn _poll<F>(&self) -> Status where F: Future + 'static {
        let task: &Task<F> = self.task.downcast_ref().unwrap();
        task.poll()
    }
}