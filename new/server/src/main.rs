use serialport::{self, SerialPortType};

fn main() {
    match serialport::available_ports() {
        Err(e) => {
            eprintln!("Error listing serial ports:");
            eprintln!("{e:?}");
        }
        Ok(mut available_ports) => {
            available_ports.sort_by_key(|key| key.port_name.clone());

            for port in available_ports {
                println!("{}", port.port_name);

                match port.port_type {
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
        }
    }
}
