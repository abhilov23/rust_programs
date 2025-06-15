use clap::Command; //helpful in building a CLI interface

fn main(){
    // Get command line arguments
    let matches = Command::new("echor").version("1.2.3").author("abhilov gupta <abhilovgupta01@gmail.com>").about("rust echo").get_matches();
}
