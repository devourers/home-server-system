# Home server system -- a very basic remote control for GNU/Linux (and partially Windows) systems.

## Installation
Clone this repo on all the devices you need, run `cargo build --release`, and you will have two executables in `target` folder -- `client` and `server`. 

## Usage
In order to use this programm, you need to have at least one config file for your server. Config is a JSON-file, which describes commands that the server can do, a small example can be found in the repo. There are 3 main types of commands:
1. `Exec` -- execute a single command with given parameters, i. e. `echo Hello`, `i3lock -i /home/user/path/to/picture.png`, etc.
2. `Sync` -- the main draw of this programm -- sync files between to machines in 2 folders. For example, you have a study folder on your main PC, and on your laptop, and you want update all of the files on PC with files from laptop. In order for this to work, you will need an aditional config file for your client. The sync works as follows -- it checks, which file is newer, and replaces the older one, while keeping all the folder structure, i.e. if PC has file `1.txt` which is newer than same file on laptop, and laptop has `2.txt` which is newer than same file on PC, after the sync both machines will have newer versions of the respectable files.
3. `Send` -- send file to another machine, for a single file transfer.

