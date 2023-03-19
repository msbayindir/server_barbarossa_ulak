

use tokio::{net::TcpStream, io::{AsyncReadExt, AsyncWriteExt}};

use crate::message::models::{Message};
const MAVEN_SERVER_ADDRESS:&str = "127.0.0.1:8001";
const CELLIYE_SERVER_ADDRESS:&str = "127.0.0.1:8002";

pub async fn handle_connection(mut stream:TcpStream){
    let mut buffer =[0;1024];
    let len = stream.read(&mut buffer).await.unwrap();

    let message =String::from_utf8_lossy(&buffer[..len]);

    let mess:Message = serde_json::from_slice(&message.to_string().as_bytes()).unwrap();
    match mess {
        mess if mess.to==124=>call_celliye(mess).await,
        mess if mess.to==123=>call_maven(mess).await,
        _ =>println!("Böyle Bir Clinent yok")
    }
    // println!("{:?}",mess);
    // if mess.to == 124{
    //     call_celliye(mess).await;
    // }else if mess.to==123{
    //     call_maven(mess).await;
    // }

}
pub async fn call_celliye(mess:Message){

    if let Ok(mut stream) = TcpStream::connect(CELLIYE_SERVER_ADDRESS).await {
       
       let bytes = mess.to_string();
       
        let _ = stream.write_all(&bytes.as_bytes()).await;

    } else {
        println!(
            "couldn't connect to celliye: {}",

            CELLIYE_SERVER_ADDRESS
        );
    }
}
pub async fn call_maven(mess:Message){

    if let Ok(mut stream) = TcpStream::connect(MAVEN_SERVER_ADDRESS).await {
       
       
       let bytes = mess.to_string();
       
        let _ = stream.write_all(&bytes.as_bytes()).await;

    } else {
        println!(
            "couldn't connect to celliye: {}",

            CELLIYE_SERVER_ADDRESS
        );
    }
}