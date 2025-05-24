use esp_idf_svc::hal::prelude::*;
use esp_idf_hal::{
    delay::BLOCK,
    i2c::{I2cConfig, I2cDriver, I2cSlaveConfig, I2cSlaveDriver},
    gpio::AnyIOPin,
};

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    const SLAVE_ADDR: u8 = 0x22;
    const SLAVE_BUFFER_SIZE: usize = 256;

    // Explicitly convert pins to AnyIOPin
    let sda_slave: AnyIOPin = peripherals.pins.gpio18.into();
    let scl_slave: AnyIOPin = peripherals.pins.gpio19.into();
    let sda_master: AnyIOPin = peripherals.pins.gpio21.into();
    let scl_master: AnyIOPin = peripherals.pins.gpio22.into();

    // Initialize I2C slave (I2C1)
    let slave_config = I2cSlaveConfig::new()
        .rx_buffer_length(SLAVE_BUFFER_SIZE)
        .tx_buffer_length(SLAVE_BUFFER_SIZE);
    let mut i2c_slave = I2cSlaveDriver::new(
        peripherals.i2c1,
        sda_slave,
        scl_slave,
        SLAVE_ADDR,
        &slave_config,
    ).unwrap();

    // Initialize I2C master (I2C0)
    let master_config = I2cConfig::new().baudrate(100.kHz().into());
    let mut i2c_master = I2cDriver::new(
        peripherals.i2c0,
        sda_master,
        scl_master,
        &master_config,
    ).unwrap();

    // Test data
    let tx_buf: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];

    // Master write test
    println!("Master write test");
    i2c_master.write(SLAVE_ADDR, &tx_buf, BLOCK).unwrap();
    let mut rx_buf: [u8; 8] = [0; 8];
    println!("Slave read test");
    i2c_slave.read(&mut rx_buf, BLOCK).unwrap();
    println!("Master write test: {:?} -> {:?}", tx_buf, rx_buf);

    // Master read test
    println!("Slave write test");
    i2c_slave.write(&tx_buf, BLOCK).unwrap();
    let mut rx_buf: [u8; 8] = [0; 8];
    println!("Master read test");
    i2c_master.read(SLAVE_ADDR, &mut rx_buf, BLOCK).unwrap();
    println!("Master read test: {:?} -> {:?}", tx_buf, rx_buf);

} 