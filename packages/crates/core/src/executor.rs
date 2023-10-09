use crate::prelude::*;

use crate::channel;

type TaskSender = Sender<Arc<Task>>;

pub fn spawner_and_executor() -> (Spawner, Executor) {
    let (task_sender, task_queue) = channel::channel();
    let spawner = Spawner { task_sender };
    let executor = Executor { task_queue };
    (spawner, executor)
}

#[derive(Debug)]
#[must_use]
struct Task {
    inner: Mutex<TaskInner>,
}

#[must_use]
struct TaskInner {
    future: Option<BoxFuture>,
    task_sender: TaskSender,
}

impl Debug for TaskInner {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("TaskInner").field("task_sender", &self.task_sender).finish()
    }
}

#[derive(Clone, Debug)]
#[must_use]
pub struct Spawner {
    task_sender: TaskSender,
}

impl Spawner {
    pub fn spawn(&self, f: impl Future<Output = ()> + Send + 'static) {
        let future = Box::pin(f) as _;
        let future = Some(future);
        let task = TaskInner { future, task_sender: self.task_sender.clone() };
        let task = Mutex::new(task);
        let task = Task { inner: task };
        let task = Arc::new(task);
        self.task_sender.send(task)
    }
}

#[derive(Debug)]
#[must_use]
pub struct Executor {
    task_queue: Receiver<Arc<Task>>,
}

impl futures_task::ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self
            .inner
            .lock()
            .task_sender
            .send(cloned)
    }
}

impl Executor {
    pub fn run_all(&self) {
        // While there are tasks to be processed
        while let Ok(task) = self.task_queue.try_recv() {
            //// Unlock the future in the Task
            let future_slot = &mut task.inner.lock().future;

            // Take it, replace with None, ...
            if let Some(mut future) = future_slot.take() {
                let waker = futures_task::waker_ref(&task);
                let context = &mut Context::from_waker(&waker);

                // ...and poll it
                if future.as_mut().poll(context).is_pending() {
                    // If it's still pending, put it back
                    *future_slot = Some(future);
                }
            }
        }
    }
}
