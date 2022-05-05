#![allow(unused_imports)] 
#![allow(dead_code)]

use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt,Signal,Process,get_current_pid,Pid};

use std::{process::{Command},time::{Instant,Duration},thread};
use std::os::windows::process::{CommandExt};
 use std::process::ExitStatus;
 use chrono::Utc;
 use colored::{Colorize,ColoredString};
fn main () {
    let mut sys = System::new_all();
    let dt = Utc::now();
    let now = Instant::now();
    // refresh before start
    sys.refresh_all();
    dt.format("%Y-%m-%d %H:%M:%S").to_string();
   println!("{}","----------------------------------MINER_MANAGER-------------------------------------".bold());
   println!("---------------------------------- {} -------------------------------------",dt.format("%Y-%m-%d %H:%M:%S").to_string().bold().blue());
   // check if there is another miner manager running before starting a new instance 
   let current_pid = get_current_pid().unwrap();
    let mut instances:Option<Pid> = None ;
    let mut running_instances =0;
   //println!(" curr pid {} {:#?}",current_pid,sys.processes_by_name("miner_manager").filter(|v|v.pid()!=current_pid).map(|e|e.pid()).collect::<Vec<Pid>>());
   sys.processes_by_name("miner_manager").filter(|v|v.pid()!=current_pid).map(|e|Some(e)).for_each(|v| { 
    
      // if  there is another, then get its pid
         match v { 
             None => {},
             Some(val) => { 
                  // increase count
                   instances = Some(val.pid());
                  
                    running_instances +=1;
             }
         }
         

   });
 //  count the running instances of the miner
  println!("running {:?}",now.elapsed());

if sys.processes_by_name("miner_manager").filter(|v|v.pid()!=current_pid).count()> 0 { 
 let _caution = "\u{26A0}\u{FE0F}".bold();
 let cross ="\u{274C}".bold();
  println!("{} {}. {}","There is an instance of this program  already running".bold().red(), format_args!("{}({})","with pid ".bold().red(),instances.unwrap().to_string().purple()),cross);
  std::process::exit(0)
}
else {
    loop {
    let now = Instant::now();
  

// First we update all information of our `System` struct.
sys.refresh_all();



// find all qpu intensive processes by
 let gpu_intensive_count = sys.processes().values().filter(|e|e.name().contains("mpc")||e.name().contains("madHC")||e.name().contains("RocketLeague")||e.name().contains("left4dead") ||e.name().to_ascii_lowercase().contains("valorant")).count();
 println!("{}{}:","processes with high GPU usage: ".bold().truecolor(173,216,230),gpu_intensive_count.to_string().truecolor(173,216,230));
   
if gpu_intensive_count >0 { 
// find and stop miner
for process in sys.processes_by_name("t-rex").map(|e|Some(e)) { 
  
     match process { 
         Some(r) => { 
 // killed all t-rex processes
     if  r.kill()  { 
         let tick = "\u{2705}";
     println!(" stopped mining process id {} : name {}  {}",r.pid().to_string().cyan(),r.name().cyan(),tick.bold());
     }
         },
         _ => { 
             // do nothing

             println!("{}","Its game or movie time, no mining".yellow())
         }
     }
}


}
else { 
// if there are no gpu intensive process, continue mining our restart mining if there is none

 if sys.processes_by_name("t-rex").count()  <1  { 



  
//restart mining 
Command::new("cmd")
.args(["/C","C:\\Users\\Spencer\\Desktop\\miner\\t-rex.exe  -a ethash -o stratum+tcp://eu1.ethermine.org:4444 -u 0x86AE5f19c53bF3408A89ABFae2aE7670DC366B57 -p x -w gtwx10606gb --mt 3  --pl 100 --fan 69   --cclock 100 --mclock 500"])
.creation_flags(0x00000010) // CREATE_NEW_CONSOLE

.spawn()
 .expect("process not found or wrong arguments");

 
 let emoji =  "\u{26CF}\u{FE0F}";
    println!("restarted mining : {}",emoji);
    
        
    
}
else  { 
    // do nothing
     let emoji =  "\u{26CF}\u{FE0F}";
    println!("{} {}","Mining still process ".purple(),emoji.bold());
}
}


// sleep for  10 seconds
thread::sleep(Duration::from_secs(10));

println!(" Executed in : {:?}",now.elapsed());
    }
}
}

