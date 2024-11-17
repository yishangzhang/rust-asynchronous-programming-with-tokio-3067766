
use std::sync::Arc;

use tokio::sync::RwLock;
use tokio::time::{Duration,sleep};

async fn read_example (id : i32, document:Arc<RwLock<String>>){
    let reader = document.read().await;
    println!("{id} read the document: {}" , reader);
}

async fn write_example (docmuent:Arc<RwLock<String>>, new_string: &str) {
    let mut  writer = docmuent.write().await;
    writer.push_str(new_string);
    writer.push_str(" ");
}

#[tokio::main]
async fn main() {
    let document = Arc::new(RwLock::new("".to_string()));

    let mut handles = Vec::new();

    for new_str in "I can speak a lot of words, o p q r s t u v w x y z".split_whitespace() {
        handles.push(tokio::spawn(read_example(1, document.clone()))); 
        handles.push(tokio::spawn(write_example(document.clone(), new_str)));
        sleep(Duration::from_millis(6)).await;
        handles.push(tokio::spawn(read_example(2, document.clone())));
        handles.push(tokio::spawn(read_example(3, document.clone())));
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
