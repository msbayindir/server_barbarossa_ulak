use async_std::{net::TcpListener, stream::StreamExt,task};
mod connection;
use connection::connection_handler;

#[async_std::main]
async fn main() {
        let listener = TcpListener::bind("127.0.0.1:8000").await;
        println!("Dinleme Yapiliyor");
        match listener {
            
            Ok(listener)=>{
                while let Some(stream) = listener.incoming().next().await{
                println!("Bağlanti Geldi");
                task::spawn(async move {
                    
                    match stream {
                        Ok(stream)=>{
                            
                           connection_handler(stream).await;
                        
                        }
                        Err(e)=>println!("Bağlanti sirasinda hata olustu:{}",e)
                    }
                } );}
                    
            }
            Err(e)=>println!("Bağlanirken Hata Olustu : {}",e)
        }


}
