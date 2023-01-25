// trying out the io_bluetoot crate
#![allow(dead_code)]
use anyhow::{Result,anyhow};
use io_bluetooth::bt::{self,BtStream};
use std::time::Instant;
use std::{io,iter};

fn main() -> Result<()> {
    let now = Instant::now();
    list_devices()?;
    println!("{:?}", now.elapsed());
    Ok(())
}

fn list_devices() -> Result<()> {
    let devices = bt::discover_devices()?;
    for (idx,device) in devices.iter().enumerate() {
        println!(" {} {}",idx,device);
    }
    // if there is no device return Error;
    if devices.is_empty() {
        return Err(anyhow!("No devices found"));
    }

    // find device_idx;
    let device_idx = request_device_idx(devices.len())?;

    let socket = BtStream::connect(iter::once(&devices[device_idx]),bt::BtProtocol::RFCOMM)?;

    match socket.peer_addr() { 
        Ok(name) => println!("Peername:{}",name.to_string()),
        Err(err) => println!(" An error occured while retrieving device name {:?}",err)
    }


    match socket.local_addr() {
        Ok(name) => println!("Socket name: {}", name.to_string()),
        Err(err) => println!("An error occured while retrieving the sockname: {:?}", err),
    }

    let mut buffer = vec![0; 1024];
    loop {
        match socket.recv_from(&mut buffer[..]) {
            Ok((len,add)) => println!("Received {} {} bytes.", len,add),
            Err(err) => return Err(anyhow!(err)),
        }
        
        match socket.send(&mut buffer[..]) {
            Ok(len) => println!("sent {} bytes.", len),
            Err(err) => return Err(anyhow!(err)),
        }
        // println!("")
        //println!("{:?}" ,String::from_utf8(buffer[..].to_vec()))
    }

    
}
fn request_device_idx(len: usize) -> Result<usize> {
    println!("Please specify the index of the Bluetooth device you want to connect to:");

    let mut buffer = String::new();
    loop {
        io::stdin().read_line(&mut buffer)?;
        if let Ok(idx) = buffer.trim_end().parse::<usize>() {
            if idx < len {
                return Ok(idx);
            }
        }
        buffer.clear();
        println!("Invalid index. Please try again.");
    }
}