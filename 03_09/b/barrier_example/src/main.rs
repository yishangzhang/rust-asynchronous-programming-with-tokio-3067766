use core::task;
use std::sync::Arc;

use tokio::sync::{Barrier,BarrierWaitResult,Notify};
use tokio::time::{Duration,sleep};

async fn  barrier_example(barrier : Arc<Barrier>, notify: Arc<Notify>, task_id : u32) -> BarrierWaitResult {
    println!("{task_id} is waitting for barrer");
    
    let task_result =  barrier.wait().await;

    println!("{task_id} passthrough the barrier" );

    if task_result.is_leader() {
        notify.notify_one();
    }
    task_result
    
}

#[tokio::main]
async fn main() {
    let cans_of_need = 12;
    let barrier = Arc::new(Barrier::new(cans_of_need));
    let notify = Arc::new(Notify::new());

    notify.notify_one();
    let mut task_handles = Vec::new();

    for task_id in 0..60 {
        if task_id % 12 == 0  {
            notify.notified().await;
            sleep(Duration::from_millis(10)).await;
        }
        
        task_handles.push(tokio::spawn(barrier_example(barrier.clone(), notify.clone(),task_id)));
    }

    let mut num_of_leaders = 0;
    for task_handle in task_handles {
        let task_result = task_handle.await.unwrap();
        if task_result.is_leader() {
            num_of_leaders +=1;
        }
    }

    println!("this day have  {num_of_leaders} of leaders");
}
