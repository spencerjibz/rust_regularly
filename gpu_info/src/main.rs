#![allow(dead_code,unused_variables, unused_imports)]
use nvml_wrapper::enum_wrappers::device::{Clock, TemperatureSensor};
use nvml_wrapper::error::NvmlError;
use nvml_wrapper::{cuda_driver_version_major, cuda_driver_version_minor,enums::device::UsedGpuMemory};
use pretty_bytes::converter::convert;
use nvml_wrapper::Nvml;
use sysinfo::{ProcessExt, RefreshKind, System, SystemExt,ProcessRefreshKind,Pid};
use std::time::Instant;
use  once_cell::sync::Lazy;
#[allow(non_upper_case_globals)]
     static nvml:Lazy<Nvml> = Lazy::new(|| {
        Nvml::init().ok().unwrap()
     });
fn main() -> Result<(), NvmlError> {
    let now = Instant::now();
    
    //let nvml = NVML::init()?;

    

    // Grabbing the first device in the system, whichever one that is.
    // If you want to ensure you get the same physical device across reboots,
    // get devices via UUID or PCI bus IDs.
    let device = nvml.device_by_index(0)?;

    // Now we can do whatever we want, like getting some data...
    let name = device.name()?;
    let temperature = device.temperature(TemperatureSensor::Gpu)?;
    let mem_info = device.memory_info()?;
    let graphics_clock = device.clock_info(Clock::Graphics)?;
    let mem_clock = device.clock_info(Clock::Memory)?;
     let device_processes = device.running_compute_processes()?;
    let system = System::new_with_specifics(RefreshKind::new().with_processes(ProcessRefreshKind::everything()));
    
     let mut process_info = vec![];
     for device_process in device_processes { 
         let process = system.process ( Pid::from(device_process.pid as usize));
      
        
    
          process_info.push(format!("{:?} ",process))
     }
      let util_rates = device.utilization_rates()?.gpu;
     let encoder_util = device.encoder_utilization()?.utilization;
         println!(" GPU:{name}  Temperature:{temperature} overall utilization: {util_rates} encoder_utilization: {encoder_util}%");
         println!("--------------------------Compute Processes ------------------------------");
         for v in  device.running_compute_processes()?{ 
             let p = system.process(Pid::from(v.pid as usize));
              println!("{:#?} ",p.unwrap().name());
              //println!("{:?} % ", util_rates)
         }
         println!("----------------------------- GRAPHICS_PROCESSES -------------------------------------");
          for graphics_process in device.running_graphics_processes()? { 
            let process = system.process ( Pid::from(graphics_process.pid as usize));
      
           
             println!("{:#?}", process.unwrap().name());
          }
      println!("{:?}",now.elapsed());

    Ok(())
}
