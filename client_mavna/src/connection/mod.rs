use async_std::io::{ReadExt, WriteExt};
#[warn(dead_code)]

use async_std::{io::stdin, net::TcpStream, task};


pub async fn on_connection(){
    let stream = TcpStream::connect("127.0.0.1:8000").await;

    match stream {
        
        Ok(mut stream)=>{
          
            
            println!("Bağlanma Başarili");
            task::spawn(async move {
                
                handle_connection(stream).await;
            });
            
            
        },
        Err(e)=>println!("hata oluştu {}",e)
    }
    
}

pub async fn handle_connection(mut stream:TcpStream){
    let mut buf = [0;1024];
    match stream.read(&mut buf).await{
        Ok(0)=>{
            println!("Herhangi bir okuma yapilmadi");
            println!("Ne soylemek istersiniz");
            let mut str = String::new();
            let bytes = stdin().read_line(&mut str).await.unwrap();
            stream.write(str.as_bytes()).await;
        }
        Ok(n)=>{
            println!("{} byte okundu",n);
            let str = String::from_utf8_lossy(&buf);
            println!("Gelen veri : ==>{}",str);
            
        }
        Err(_) => todo!(),
    }
}
