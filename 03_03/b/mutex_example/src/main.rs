
use std::sync::Arc;

use tokio::sync::Mutex;
use tokio::time::{Duration,sleep};


async fn person(remoterc: Arc<Mutex<i32>>, name: String, new_chanel:i32){
    let mut tv_channel = remoterc.lock().await;
    *tv_channel = new_chanel;
    println!("{name} change the channel");
    println!{"channel is {new_chanel} now"};
    sleep(Duration::from_secs(2)).await;
}


#[tokio::main]
async fn main() {
    let tv_chennel = 32;
    let remote = Mutex::new(tv_chennel);
    let remote_arc = Arc::new(remote);
    let mut spawn_handles = Vec::new();
    for (name,newchannel) in [("lilei",34),("wangyangming",18),("zengguopan",32)] {
        spawn_handles.push(tokio::spawn(person(remote_arc.clone(), name.to_string(), newchannel)));
    }

    for handle in spawn_handles {
        handle.await.unwrap();
    }

}
