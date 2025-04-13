use crate::{Elder, RelayAddress};
use serialport::{DataBits, SerialPort, SerialPortType, StopBits};
use std::io::Write as _;

pub struct PooferBusPort {
    port: Box<dyn SerialPort>,
}
impl PooferBusPort {
    pub fn available_ports() -> Vec<String> {
        match serialport::available_ports() {
            Err(e) => {
                eprintln!("Error listing serial ports:");
                eprintln!("{e:?}");
                ::std::process::exit(1);
            }
            Ok(mut available_ports) => {
                available_ports.sort_by_key(|key| key.port_name.clone());

                available_ports
                    .iter()
                    .filter_map(|port| {
                        println!("{}", port.port_name);

                        match &port.port_type {
                            SerialPortType::UsbPort(info) => {
                                println!("  Type: USB");
                                println!("  VID: {:04x}", info.vid);
                                println!("  PID: {:04x}", info.pid);
                                println!(
                                    "  Serial Number: {}",
                                    info.serial_number.as_ref().map_or("", String::as_str)
                                );
                                println!(
                                    "  Manufacturer: {}",
                                    info.manufacturer.as_ref().map_or("", String::as_str)
                                );
                                println!(
                                    "  Product: {}",
                                    info.product.as_ref().map_or("", String::as_str)
                                );
                            }
                            SerialPortType::BluetoothPort => {
                                println!("  Type: Bluetooth");
                            }
                            SerialPortType::PciPort => {
                                println!("  Type: PCI");
                            }
                            SerialPortType::Unknown => {
                                println!("  Type: Unknown");
                            }
                        }
                        if matches!(port.port_type, SerialPortType::UsbPort(_)) {
                            Some(port.port_name.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<String>>()
            }
        }
    }
    pub fn new(serial_port_name: &str) -> Self {
        let port = serialport::new(serial_port_name, 19200)
            .stop_bits(StopBits::One)
            .data_bits(DataBits::Eight)
            .open()
            .unwrap_or_else(|e| {
                eprintln!("Failed to open port {}. Error: {}", serial_port_name, e);
                ::std::process::exit(1);
            });

        Self { port }
    }

    pub fn output(&mut self, elders: &Vec<Elder>) {
        for elder in elders {
            let on_digit = elder.poofer.on as u8;
            for RelayAddress {
                board_address,
                poofer_address,
            } in &elder.poofer.relays
            {
                let command = format!("!{board_address:02}{poofer_address}{on_digit}.");
                match self.port.write(command.as_bytes()) {
                    Ok(_) => {
                        print!("{}", command);
                        std::io::stdout().flush().unwrap();
                    }
                    Err(e) => eprintln!("{e:?}"),
                }
            }
        }
        println!();
    }
}
