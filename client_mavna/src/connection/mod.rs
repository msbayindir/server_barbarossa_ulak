

use async_std::io::{ReadExt, WriteExt};
#[warn(dead_code)]

use async_std::{io::stdin, net::TcpStream, task};
use libs::bar_stdin;

pub async fn on_connection(){
    let stream = TcpStream::connect("127.0.0.1:8000").await;
    
    match stream {
        
        Ok(mut stream)=>{
          
            
            println!("Bağlanma Başarili");
            task::spawn(async move {
                let mut a = true;
                
                while a {
                   match handle_connection(stream.clone()).await{
                       Some(())=>continue,
                       None => a = false
                   } 
                }
                
            }).await;
            
            
        },
        Err(e)=>println!("hata oluştu {}",e)
    }
    
}

pub async fn handle_connection(mut stream:TcpStream)->Option<()>{
    let mut buf = [0;1024];
    println!("handle");
    match stream.read(&mut buf).await{
        Ok(0)=>{
            println!("Herhangi bir okuma yapilmadi");

            match bar_stdin::Stdin::read_line().await{
                Some(value)=>{
                    let bytes = value.user_message.as_bytes();
                    stream.write(bytes).await;
                    Some(())
                },
                None=>{
                    println!("Kullanici Giriş Yapmadi Stream Kapatiliyor");
                    None
                }

            }
            
        }
        Ok(n)=>{
            println!("{} byte okundu",n);
            let str = String::from_utf8_lossy(&buf);
            println!("Gelen veri : ==>{}",str);
            Some(())
        }
        Err(_) => {
            println!("handle");
            None
            }
    }
}
