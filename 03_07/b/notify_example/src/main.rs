use std::sync::Arc;

use tokio::sync::Notify;
use tokio::time::{Duration,sleep};

async fn ship_package(package_develiverd : Arc<Notify>) {
    println!("find package in warehourse");
    sleep(Duration::from_secs(2)).await;
    println!("ship package");
    sleep(Duration::from_secs(2)).await;
    println!("package have been develiverd");
    package_develiverd.notify_one();
}

async fn grap_package (package_develiverd : Arc<Notify>){
    package_develiverd.notified().await;
    println!("find package outside the door");
    sleep(Duration::from_secs(2)).await;
    println!("grab package");
}

#[tokio::main]
async fn main(){
    let package_develiverd = Notify::new();
    let package_develiverd_arc = Arc::new(package_develiverd);

    let ship_package_handle =  tokio::spawn(ship_package(package_develiverd_arc.clone()));

    let grap_package_handle = tokio::spawn(grap_package(package_develiverd_arc.clone()));

    //ship_package_handle.await.unwrap();
    grap_package_handle.await.unwrap();
    ship_package_handle.await.unwrap();
}