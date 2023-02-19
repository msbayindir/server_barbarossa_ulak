use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

use tokio::net::TcpListener;

const ULAK_IP_ADDRESS:&str = "127.0.0.1:8000";

#[tokio::main]
async fn main() {

    let listener = TcpListener::bind(ULAK_IP_ADDRESS).await.unwrap();


    loop {
            let (stream,_) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                handle_connection(stream).await
            });
    }

}


async fn handle_connection(mut stream:TcpStream){

    let mut buffer =[0;1024];
    let len = stream.read(&mut buffer).await.unwrap();
    let message =String::from_utf8_lossy(&buffer[..len-1]);
    println!("message :{} , len :{}",message.to_string(),len);
}
