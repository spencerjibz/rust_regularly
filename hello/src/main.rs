 use std::os::windows::process::{CommandExt};
 use std::process::{Command,ExitStatus};
fn main()  {

    Command::new("cmd")
    .args(["/K","C:\\Users\\Spencer\\Desktop\\miner\\t-rex.exe  -a ethash -o stratum+tcp://eu1.ethermine.org:4444 -u 0x86AE5f19c53bF3408A89ABFae2aE7670DC366B57 -p x -w gtwx10606gb"])
    .creation_flags(0x00000010) // CREATE_NEW_CONSOLE
    
    .spawn()
     .expect("process not found or wrong arguments");

     
    

      
      // println!("restarted mining");
}


