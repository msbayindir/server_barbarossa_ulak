use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::net::TcpListener;

mod message;
mod connection;




const ULAK_IP_ADDRESS:&str = "127.0.0.1:8000";
const MAVEN_SERVER_ADDRESS:&str = "127.0.0.1:8001";
const CELLIYE_SERVER_ADDRESS:&str = "127.0.0.1:8002";




#[tokio::main]
async fn main() {

    let listener = TcpListener::bind(ULAK_IP_ADDRESS).await.unwrap();

    loop {
            let (stream,adr) = listener.accept().await.unwrap();

            tokio::spawn(async move {
               
                connection::handle_connection(stream).await
            });
    }

}





