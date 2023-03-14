#[warn(dead_code,unused)]
use tokio::net::TcpListener;

mod message;
mod connection;




const ULAK_IP_ADDRESS:&str = "127.0.0.1:8000";


#[tokio::main]
async fn main() {

    let listener = TcpListener::bind(ULAK_IP_ADDRESS).await.unwrap();
    

    loop {
            let (stream,_) = listener.accept().await.unwrap();

            tokio::spawn(async move {
               
                connection::handle_connection(stream).await
            });
    }

}





