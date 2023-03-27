use async_std::{net::TcpStream, io::{ReadExt, WriteExt}};




pub async fn connection_handler(mut stream:TcpStream){
    // let mut buff = [0;1024];
    // stream.read(&mut buff).await;
    // let str = String::from_utf8_lossy(&buff);
    //println!("{}",str);
    stream.write_all(b"sefa").await;
}