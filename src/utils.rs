pub const PORT:u16 = 1337;

pub enum MessageType{
    HelloMsg,
    CommandMsg,
    //todo...
}

#[derive(PartialEq, Debug)]
pub enum CommandCode{
    Done,
    Sync,
    ShowVariant,
    Error
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
    //Sync:
    // first arg -- dir where to sync, 
}


impl ServerCommand{
    pub fn exec(&self) -> (CommandCode, &str){
        if self.command_type == CommandType::Exec {
            let r = std::process::Command::new(&self.command).args(&self.args).spawn();
            if r.is_ok(){
                return (CommandCode::Done, "");
            }
            else{
                return (CommandCode::Error, "");
            }
        }
        else if self.command_type == CommandType::Sync{
            return (CommandCode::Sync, &self.args[0]);
        }
        else{
            return (CommandCode::ShowVariant, &self.args[0]);
        }
    }
}


pub fn list_nested_dirs(path: &str) -> Vec<std::path::PathBuf>{
    let mut res: Vec<std::path::PathBuf> = vec!();
    let paths = std::fs::read_dir(std::path::Path::new(path)).unwrap();
    for path in paths{
        let path_cp = path.unwrap();
        if std::fs::metadata(&path_cp.path()).unwrap().is_dir(){
            let new_res = list_nested_dirs(&path_cp.path().to_str().unwrap());
            for elem in new_res{
                res.push(elem);
            }
        }
        else{
            res.push(path_cp.path());
        }
    }
    return res;
}

pub fn absolute_to_relative_path(path_abs: &String, path_dir: &String)-> String{
    let splitted_path_abs: Vec<&str> = path_abs.split('/').collect();
    let splitted_path_sync: Vec<&str> = path_dir.split('/').collect();
    let mut rel_path = "".to_string();
    for i in splitted_path_abs{
        if splitted_path_sync.contains(&i) == false{
            rel_path += i;
            rel_path += "/";
        }
    }
    rel_path = rel_path[0..rel_path.len() -1].to_string();
    return rel_path;
}

pub fn form_sync_response(command0: String, files: &Vec<std::path::PathBuf>, sync_loc: &String ) -> String{
    let mut msg = command0 + &"|".to_string();
    for file in files{
        let file_content = std::fs::read(&file).unwrap();
        let rel_path = absolute_to_relative_path(&file.display().to_string(), &sync_loc);
        let last_mod = std::fs::metadata(&file).unwrap().modified().unwrap().elapsed().unwrap().as_secs().to_string();
        msg += &(rel_path + "::" + &base64::encode(file_content) + "::" + &last_mod + ";");
    }
    msg += "\n";
    return msg;
}

pub fn parse_sync_line(line: &String) -> Vec<(String, String, String)>{
    let splitted_command_body: Vec<&str> = line.split('|').collect();
    let splitted_files: Vec<&str> = splitted_command_body[1].split(';').collect();
    let mut res: Vec<(String, String, String)> = vec!();
    for file in splitted_files{
        let current_file: Vec<&str> = file.split("::").collect();
        if current_file.len() == 3{
            res.push((current_file[0].to_string(), current_file[1].to_string(), current_file[2].to_string()));
        }
    }
    return res;
}

pub fn compare_dirs(ours: &String, theirs: &String, dir_to_save: &std::path::Path){
    let mut our_files = parse_sync_line(ours);
    our_files.sort_by(|x, y| x.0.cmp(&y.0));
    let mut their_files = parse_sync_line(theirs);
    their_files.sort_by(|x, y| x.0.cmp(&y.0));
    for file_their in &their_files{
        let mut is_new = true;
        for file_our in &our_files{
            if file_our.0 == file_their.0{
                is_new = false;
                if file_their.2.parse::<i64>().unwrap() < file_our.2.parse::<i64>().unwrap(){
                    let contents = base64::decode(&file_their.1).unwrap();
                    let path = dir_to_save.clone().display().to_string() + "/" + &file_their.0;
                    std::fs::write(&path, contents).unwrap();
                }
            }
        }
        if is_new{
            let full_path_to_file = dir_to_save.clone().display().to_string() + "/" + &file_their.0;
            let path = std::path::Path::new(&full_path_to_file);
            let prefix = path.parent().unwrap();
            let contents = base64::decode(&file_their.1).unwrap();
            std::fs::create_dir_all(prefix).unwrap();
            std::fs::write(path, contents).unwrap();
        }
    }
}