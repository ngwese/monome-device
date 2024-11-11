use clap::Parser;
use monome_device::{Device, Event};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, help = "device to open")]
    name: String,
}

fn main() {
    let args = Args::parse();

    let open_result = Device::open_file(&args.name);
    if let Ok(device) = open_result {
        println!("opened: {:#?}", device);

        let _ = device.led_all(0);

        let mut event_loop = device.event_loop();
        while let Some(event) = event_loop.next() {
            println!("event: {:?}", event);
            match event {
                Event::ButtonPress { x, y } => {
                    let _ = device.led_on(x, y);
                }
                Event::ButtonLift { x, y } => {
                    let _ = device.led_off(x, y);
                }
                _ => {}
            }
        }
    } else {
        println!("Unable to open: {:?}", open_result);
    }
}
