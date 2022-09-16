pub const PORT:u16 = 1337;

pub enum MessageType{
    HelloMsg,
    ShutdownMsg,
    //todo...
}

#[derive(PartialEq, Debug, serde::Deserialize)]
pub enum CommandType{
    #[serde(rename = "Sync")]
    Sync,
    #[serde(rename = "Exec")]
    Exec,
    #[serde(rename = "Send")]
    Send
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


#[derive(Debug, serde::Deserialize)]
pub struct ServerCommand{   
    pub name: String,
    pub brief: String,
    pub command: String,
    pub args: Vec<String>,
    pub command_type: CommandType
}


impl ServerCommand{
    pub fn exec(&self){
        if self.command_type == CommandType::Exec {
            std::process::Command::new(&self.command).args(&self.args).spawn().unwrap();
        }
    }
}
