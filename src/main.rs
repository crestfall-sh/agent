
use bollard::Docker;

#[tokio::main]
async fn test() {
    println!("Hello, world!");
    let docker = Docker::connect_with_socket_defaults().unwrap();
    let version = docker.version().await.unwrap();
    println!("{:?}", version);  
}

use tokio_tungstenite::{connect_async};

#[tokio::main]
async fn test2() {
    let asd = "ws://localhost:8080/";
    let url = url::Url::parse(&asd).unwrap();
    let (socket, _) = connect_async(url).await.expect("Connection failed.");
    println!("Connected.");
}

fn main() {
    test();
    test2();
}