use std::f32::consts::E;

use async_std::io::stdin;
pub struct Stdin{
    status:bool,
    user_message:Option<String>
}


impl Stdin{
    pub async fn read_line()->Option<Stdin>{
        let mut str = String::new();
        match stdin().read_line(&mut str).await{
            Ok(0)=>Some(Stdin { status: false, user_message:None }),
            Ok(n)=>Some(Stdin { status: true, user_message: Some(str) }),
            Err(e)=>None
        }
     }
    
}