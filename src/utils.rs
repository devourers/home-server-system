pub const PORT:u16 = 8080;

pub enum MessageType{
    HelloMsg,
    ShutdownMsg,
    //todo...
}

pub struct Message{
    pub body: String,
    pub sender: String,
    pub msg_type: MessageType,
}

impl Message{
    pub fn new(body: &String, sender: &String, msg_type: MessageType) -> Self{
        return Message{ body: body.to_string(),
                        sender: sender.to_string(),
                         msg_type};
    }

    pub fn to_string(self) -> String{
        let mut msg = String::new();
        msg.push_str(&self.sender);
        msg.push_str(": ");
        msg.push_str(&self.body);
        return msg;
    }
}

