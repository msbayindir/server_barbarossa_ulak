#[warn(dead_code,unused)]
use serde_derive::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Message{
    pub owner:String,
    pub to:i32,
    pub message:String
}
impl Message{

    // pub fn new(message:String,owner:String,to:i32)->Self{
    //   return  Self{
    //         owner:owner,
    //         message:message,
    //         to:to
    //     };
    // }
    pub fn to_string(&self)->String{
    return  serde_json::to_string(&self).unwrap();
    
    
    }

}