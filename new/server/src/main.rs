use serialport::{self, DataBits, SerialPortType, StopBits};
use std::io::Write as _;

fn main() {
    match serialport::available_ports() {
        Err(e) => {
            eprintln!("Error listing serial ports:");
            eprintln!("{e:?}");
        }
        Ok(mut available_ports) => {
            available_ports.sort_by_key(|key| key.port_name.clone());

            for port in &available_ports {
                println!("{}", port.port_name);

                match &port.port_type {
                    SerialPortType::UsbPort(info) => {
                        println!("        Type: USB");
                        println!("        VID: {:04x}", info.vid);
                        println!("        PID: {:04x}", info.pid);
                        println!(
                            "        Serial Number: {}",
                            info.serial_number.as_ref().map_or("", String::as_str)
                        );
                        println!(
                            "        Manufacturer: {}",
                            info.manufacturer.as_ref().map_or("", String::as_str)
                        );
                        println!(
                            "        Product: {}",
                            info.product.as_ref().map_or("", String::as_str)
                        );
                    }
                    SerialPortType::BluetoothPort => {
                        println!("        Type: Bluetooth");
                    }
                    SerialPortType::PciPort => {
                        println!("        Type: PCI");
                    }
                    SerialPortType::Unknown => {
                        println!("        Type: Unknown");
                    }
                }
            }

            if let Some(port) = available_ports.get(0) {
                let mut port = serialport::new(&port.port_name, 19200)
                    .stop_bits(StopBits::One)
                    .data_bits(DataBits::Eight)
                    .open()
                    .unwrap_or_else(|e| {
                        eprintln!("Failed to open port {}. Error: {}", port.port_name, e);
                        ::std::process::exit(1);
                    });

                let command = "!0121.";
                match port.write(command.as_bytes()) {
                    Ok(_) => {
                        print!("{}", command);
                        std::io::stdout().flush().unwrap();
                    }
                    Err(e) => eprintln!("{e:?}"),
                }
                println!();
            }
        }
    }
}
