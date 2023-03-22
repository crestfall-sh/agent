
use bollard::Docker;

#[tokio::main]
async fn test() {
    println!("Hello, world!");
    let docker = Docker::connect_with_socket_defaults().unwrap();
    let version = docker.version().await.unwrap();
    println!("{:?}", version);  
}

use futures_util::{future, pin_mut, SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{
    connect_async,
    tungstenite::protocol::Message,
    tungstenite::{Error, Result}
};

#[tokio::main]
async fn test2() -> Result<()>{

    let url = url::Url::parse("ws://localhost:8080/").unwrap();

    let (mut socket, _) = connect_async(url).await.expect("Connection failed.");
    println!("Connected.");

    let hello = socket.next().await.unwrap().unwrap();
    println!("Message received.");
    println!("{:?}", hello.into_text().unwrap());

    let hi = Message::Text("HI".into());
    socket.send(hi).await.expect("Message send failed.");
    println!("Message sent.");


    while let Some(msg) = socket.next().await {
        let message = msg?;
        if message.is_text() == true {
            let message_text = message.into_text().unwrap();
            println!("{:?}", message_text);
            let response = Message::Text("TEST".into());
            socket.send(response).await?;
        }
    }
    Ok(())

}

fn main() -> Result<()> {
    test();
    test2()?;
    Ok(())
}