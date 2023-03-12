use tokio::{net::TcpStream, io::{AsyncReadExt, AsyncWriteExt}};

use crate::message::models::Message;

pub async fn handle_connection(mut stream:TcpStream){
    let mut buffer =[0;1024];
    let len = stream.read(&mut buffer).await.unwrap();

    let message =String::from_utf8_lossy(&buffer[..len]);

    let gelen:Message = serde_json::from_slice(&message.to_string().as_bytes()).unwrap();
    println!("{:?}",gelen.to);
   
    stream.write_all(&mut message.as_bytes()).await.unwrap();


}