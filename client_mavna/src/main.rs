use async_std::{net::{TcpStream}, stream, io::{WriteExt, ReadExt}};
use async_std::io::stdin;

#[async_std::main]
async fn main() {

    let stream = TcpStream::connect("127.0.0.1:8000").await;

    match stream {
        
        Ok(mut stream)=>{
            while true {
                
            
            println!("Bağlanma Başarili");
            let mut str = String::new();
            let bytes = stdin().read_line(&mut str).await.unwrap();
            stream.write(str.as_bytes()).await;
            }
        },
        Err(e)=>println!("hata oluştu {}",e)
    }
    
}   
