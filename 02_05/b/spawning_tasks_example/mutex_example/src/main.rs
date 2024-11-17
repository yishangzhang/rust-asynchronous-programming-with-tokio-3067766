use std::{thread,time::{self, Duration}};

fn block_code() -> String {
    thread::sleep(time::Duration::from_secs(1));
    "123hello spwan".to_string()
}

async fn async_code(id:u32) {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("async id {id}");
}

fn blcok_code_wrap(name : String) -> impl FnOnce()->String + Send + 'static {
    move || {
        thread::sleep(Duration::from_secs(1));
        println!("block_code_wrap {}",name);
        "123hello wrap spawn".to_string()
    }
}


#[tokio::main]
async fn main() {
    let block_handles = [tokio::task::spawn_blocking(block_code), tokio::task::spawn_blocking(blcok_code_wrap("name from main".to_string()))];

    

    let mut async_handles = Vec::new();
    for i in 0..10 {
        async_handles.push(tokio::spawn(async_code(i)));
    }

    for async_handle in async_handles {
        async_handle.await.unwrap();
    }
    let mut res;

    for block_handle in block_handles {
        res = block_handle.await.unwrap();
        println!("res is {}",res);
    }

    
}
