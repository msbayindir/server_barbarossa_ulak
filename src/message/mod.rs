mod models;
use models::Message;

pub fn new(message:String,to:i32,owner:String)->Message
{
       return  Message{message:message,to:to,owner:owner};
}

