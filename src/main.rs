use eyre::Result;
use serialport::{available_ports, open_with_settings, SerialPort, SerialPortSettings};
use std::time::Duration;

pub fn open_port(port: String) -> Result<Box<dyn SerialPort>> {
    Ok(open_with_settings(
        &port,
        &SerialPortSettings {
            baud_rate: 115200,
            // data_bits: DataBits::Eight,
            // flow_control: FlowControl::None,
            // parity: Parity::None,
            // stop_bits: StopBits::One,
            timeout: Duration::from_millis(20),
            ..SerialPortSettings::default()
        },
    )?)
}

pub fn main() -> Result<()> {
    for port_info in available_ports().unwrap() {
        println!("{:?}", port_info);
        let mut port = open_port(port_info.port_name).unwrap();

        port.write(&"Hi\n".bytes().collect::<Vec<_>>())?;
        let mut serial_buf: Vec<u8> = vec![0; 32];
        println!("{:?}", port.bytes_to_read()?);
        port.read(serial_buf.as_mut_slice())?;
    }
    println!("Hi!");

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
