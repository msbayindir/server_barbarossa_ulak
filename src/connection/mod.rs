use tokio::{net::TcpStream, io::{AsyncReadExt, AsyncWriteExt}};

pub async fn handle_connection(mut stream:TcpStream){
    let mut buffer =[0;1024];
    let len = stream.read(&mut buffer).await.unwrap();
    let message =String::from_utf8_lossy(&buffer[..len-1]);

    println!("message :{} , len :{}",message.to_string(),len);
   
    stream.write_all(&mut message.as_bytes()).await.unwrap();
    stream.w

}