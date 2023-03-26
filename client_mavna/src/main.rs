use async_std::{net::{TcpStream},  io::{WriteExt}};
use async_std::io::stdin;
mod connection;
#[async_std::main]
async fn main() {
    
    let stream = TcpStream::connect("127.0.0.1:8000").await;

    match stream {
        
        Ok(mut stream)=>{
          
            
           connection::on_connection().await;
        
        },
        Err(e)=>println!("hata oluştu {}",e)
    }
    
}   
