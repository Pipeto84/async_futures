use futures::{
    future::{BoxFuture, FutureExt},
    task::{waker_ref, ArcWake},
};
use std::{
    future::Future,
    sync::mpsc::{sync_channel, Receiver, SyncSender},
    sync::{Arc, Mutex},
    task::Context,
    time::Duration,
};
use async_futures::TimerFuture;
struct Executor{
    ready_queue:Receiver<Arc<Task>>,
}
#[derive(Clone)]
struct Spawner{
    task_sender:SyncSender<Arc<Task>>,
}
struct Task{
    future:Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender:SyncSender<Arc<Task>>,
}
fn new_executor_and_spawner()->(Executor,Spawner) {
    const MAX_QUEUED_TASKS:usize=10_000;
    println!("{}:{} new_executor_and_spawner",file!(),line!());
    let (task_sender,ready_queue)=sync_channel(MAX_QUEUED_TASKS);
    println!("{}:{} new_executor_and_spawner",file!(),line!());
    (Executor{ready_queue},Spawner{task_sender})
}
impl Spawner {
    fn spawn(&self,future:impl Future<Output = ()> + 'static + Send) {
        println!("{}:{} spawn",file!(),line!());
        let future=future.boxed();
        println!("{}:{} spawn",file!(),line!());
        let task=Arc::new(Task{
            future:Mutex::new(Some(future)),
            task_sender:self.task_sender.clone(),
        });
        println!("{}:{} spawn",file!(),line!());
        self.task_sender.send(task).expect("demaciadas tareas en la cola");
        println!("{}:{} spawn",file!(),line!());
    }
}
impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        println!("{}:{} wake_by_ref",file!(),line!());
        let cloned=arc_self.clone();
        println!("{}:{} wake_by_ref",file!(),line!());
        arc_self
            .task_sender
            .send(cloned)
            .expect("muchas tareas en la cola");
        println!("{}:{} wake_by_ref",file!(),line!());
    }
}
impl Executor {
    fn run(&self) {
        println!("{}:{} run",file!(),line!());
        while let Ok(task) = self.ready_queue.recv() {
            println!("\n{}:{} run",file!(),line!());
            let mut future_slot=task.future.lock().unwrap();
            println!("{}:{} run",file!(),line!());
            if let Some(mut future) = future_slot.take() {
                println!("{}:{} run",file!(),line!());
                let waker=waker_ref(&task);
                println!("{}:{} run",file!(),line!());
                let context=&mut Context::from_waker(&waker);
                println!("{}:{} run",file!(),line!());
                if future.as_mut().poll(context).is_pending() {
                    println!("{}:{} run",file!(),line!());
                    *future_slot=Some(future);
                }
            }
            println!("{}:{} run",file!(),line!());
        }
        println!("{}:{} run",file!(),line!());
    }
}
fn main() {
    println!("{}:{} main",file!(),line!());
    let (executor,spawner)=new_executor_and_spawner();
    println!("{}:{} main",file!(),line!());
    spawner.spawn(async{
        println!("howdy!");
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
        TimerFuture::new(Duration::new(3, 0)).await;
        println!("Pipeto!");
        TimerFuture::new(Duration::new(4, 0)).await;
    });
    println!("{}:{} main",file!(),line!());
    drop(spawner);
    println!("{}:{} main",file!(),line!());
    executor.run();
    println!("{}:{} main",file!(),line!());
}