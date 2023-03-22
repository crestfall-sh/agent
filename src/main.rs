use bollard::Docker;

#[tokio::main]
async fn test() {
    println!("Hello, world!");
    let docker = Docker::connect_with_socket_defaults().unwrap();
    let version = docker.version().await.unwrap();
    println!("{:?}", version);  
}

fn main() {
    test();
}