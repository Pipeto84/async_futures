use std::{
    future::Future,
    pin::Pin,
    sync::{Arc,Mutex},
    task::{Context,Poll,Waker},
    thread,
    time::Duration,
};
pub struct TimerFuture{
    shared_state:Arc<Mutex<SharedState>>,
}
#[derive(Debug)]
struct SharedState{
    complete:bool,
    waker:Option<Waker>,
}
impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("{}:{} poll",file!(),line!());
        let mut shared_state=self.shared_state.lock().unwrap();
        if shared_state.complete {
            println!("{}:{} Poll::Ready(())",file!(),line!());
            Poll::Ready(())
        }else {
            shared_state.waker=Some(cx.waker().clone());
            println!("{}:{} Poll::Pending",file!(),line!());
            Poll::Pending
        }
    }
}
impl TimerFuture {
    pub fn new(duration:Duration)->Self {
        println!("{}:{} TimerFuture::new",file!(),line!());
        let shared_state=Arc::new(Mutex::new(SharedState{
            complete:false,
            waker:None,
        }));
        println!("{}:{} TimerFuture::new",file!(),line!());
        let thread_shared_state=shared_state.clone();
        println!("{}:{} TimerFuture::new",file!(),line!());
        thread::spawn(move||{
            println!("{}:{} TimerFuture::new::thread::spawn",file!(),line!());
            thread::sleep(duration);
            println!("{}:{} TimerFuture::new::thread::spawn",file!(),line!());
            let mut shared_state=thread_shared_state.lock().unwrap();
            println!("{}:{} TimerFuture::new::thread::spawn",file!(),line!());
            shared_state.complete=true;
            println!("{}:{} TimerFuture::new::thread::spawn",file!(),line!());
            if let Some(waker) = shared_state.waker.take() {
                println!("{}:{} TimerFuture::new::thread::spawn",file!(),line!());
                waker.wake()
            }
            println!("{}:{} TimerFuture::new::thread::spawn",file!(),line!());
        });
        println!("{}:{} TimerFuture::new -> TimerFuture.shared_state",file!(),line!());
        TimerFuture { shared_state }
    }
}