use std::f32::consts::E;

use async_std::io::stdin;
pub struct Stdin{
    pub status:bool,
    pub user_message:String
}


impl Stdin{
    pub async fn read_line()->Option<Stdin>{
        println!("Ne Söylemek İstersiniz");
        let mut str = String::new();
        match stdin().read_line(&mut str).await{
            Ok(0)=>None,
            Ok(n)=>Some(Stdin { status: true, user_message: str}),
            Err(e)=>None
        }
     }
    
}