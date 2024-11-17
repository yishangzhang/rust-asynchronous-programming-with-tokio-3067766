use std::sync::Arc;

use tokio::sync::Semaphore;
use tokio::time::{Duration,sleep};

async fn people(sem : Arc<Semaphore> , name : String) {

    println!("{name} is waiting for served");
     teller(sem, name).await;
}

async fn teller(sem: Arc<Semaphore> , name :String){
    let permit = sem.acquire().await.unwrap();
    sleep(Duration::from_secs(1)).await;
    println!("{name} is being served");
    sleep(Duration::from_secs(2)).await;
    drop(permit);
    println!("{name} is leaving");
}


#[tokio::main]
async fn main(){
    let totol = 4;
    let sem = Semaphore::new(totol);
    let sem_arc = Arc::new(sem);

    let mut handles = Vec::new();

    for i in 0..10 {
        handles.push(tokio::spawn(people(sem_arc.clone(), format!("person_{i}"))));
    }

    for handle in handles {
        handle.await.unwrap();
    }
}