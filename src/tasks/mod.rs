use core::task::{
    Waker,
    Context,
    Poll,
};

use x86_64::instructions::interrupts;
use alloc::{
    collections::BTreeMap,
    sync::Arc,
    task::Wake
};
use crossbeam_queue::ArrayQueue;
use lazy_static::lazy_static;
use spin::Mutex;

pub mod task;
use task::{
    Task,
    TaskId
};


lazy_static! {
    pub static ref EXECUTOR : Mutex<Executor> = Mutex::new(Executor::new());
}


pub fn init() {
    let mut executor = EXECUTOR.lock();
    executor.spawn(crate::tasks::task::Task::new(crate::tasks::task::keyboard::print_keypresses()));
    executor.run();
}


pub struct Executor {
    tasks       : BTreeMap<TaskId, Task>,
    task_queue  : Arc<ArrayQueue<TaskId>>,
    waker_cache : BTreeMap<TaskId, Waker>
}
impl Executor {
    pub fn new() -> Executor {
        return Executor {
            tasks       : BTreeMap::new(),
            task_queue  : Arc::new(ArrayQueue::new(100)),
            waker_cache : BTreeMap::new()
        };
    }
    pub fn spawn(&mut self, task : Task) {
        let task_id = task.id;
        if (self.tasks.insert(task.id, task).is_some()) {
            panic!("Attempted to use already active task id.");
        }
        self.task_queue.push(task_id).expect("Attempted to queue task when task queue is full.");
    }
    pub fn run(&mut self) -> ! {
        loop {
            self.run_ready();
            interrupts::disable();
            if (self.task_queue.is_empty()) {
                interrupts::enable_and_hlt()
            } else {
                interrupts::enable();
            }
        }
    }
    fn run_ready(&mut self) {
        let Self {
            tasks,
            task_queue,
            waker_cache
        } = self;
        while let Ok(task_id) = task_queue.pop() {
            let task = match tasks.get_mut(&task_id) {
                Some(task) => task,
                None       => continue,
            };
            let waker = waker_cache
                .entry(task_id)
                .or_insert_with(|| TaskWaker::new(task_id, task_queue.clone()));
            let mut context = Context::from_waker(waker);
            match (task.poll(&mut context)) {
                Poll::Ready(()) => {
                    tasks.remove(&task_id);
                    waker_cache.remove(&task_id);
                },
                Poll::Pending => {}
            }
        }
    }
}
unsafe impl Send for Executor {}


struct TaskWaker {
    task_id    : TaskId,
    task_queue : Arc<ArrayQueue<TaskId>>
}
impl TaskWaker {
    fn new(task_id : TaskId, task_queue : Arc<ArrayQueue<TaskId>>) -> Waker {
        return Waker::from(Arc::new(TaskWaker {
            task_id,
            task_queue
        }));
    }
    fn wake_task(&self) {
        self.task_queue.push(self.task_id).expect("Attempted to wake task when task queue is full.");
    }
}
impl Wake for TaskWaker {
    fn wake(self : Arc<Self>) {
        self.wake_task();
    }
    fn wake_by_ref(self : &Arc<Self>) {
        self.wake_task();
    }
}
