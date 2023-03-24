
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Message{
     owner:String,
     to:i32,
     message:String
}


impl Message{

    pub fn new(message:String,owner:String,to:i32)->Self{
      return  Self{
            owner:owner,
            message:message,
            to:to
        };
    }
    // pub fn to_string(&self)->String{
    // return  serde_json::to_string(&self).unwrap();
    
    
    // }
}
