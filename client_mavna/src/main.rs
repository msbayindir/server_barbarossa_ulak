use async_std::{net::{TcpStream},  io::{WriteExt}};
use async_std::io::stdin;
mod connection;
#[async_std::main]
async fn main() {
    
   connection::on_connection().await;
    
}   
