use esp_idf_svc::hal::prelude::*;
use esp_idf_hal::{
    delay::{BLOCK, FreeRtos},
    i2c::{I2cConfig, I2cDriver},
    gpio::AnyIOPin,
};

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    println!("Starting I2C OLED display test...");
    
    let peripherals = Peripherals::take().unwrap();

    // OLED display typically uses address 0x3C
    const OLED_ADDR: u8 = 0x3C;

    // Explicitly convert pins to AnyIOPin
    let sda: AnyIOPin = peripherals.pins.gpio19.into();
    let scl: AnyIOPin = peripherals.pins.gpio21.into();

    println!("Initializing I2C master...");
    // Initialize I2C master (I2C0)
    let master_config = I2cConfig::new().baudrate(100.kHz().into());
    let mut i2c = match I2cDriver::new(
        peripherals.i2c0,
        sda,
        scl,
        &master_config,
    ) {
        Ok(driver) => {
            println!("I2C master initialized successfully");
            driver
        }
        Err(e) => {
            println!("Failed to initialize I2C master: {:?}", e);
            return;
        }
    };

    // Add a delay after initialization
    FreeRtos::delay_ms(100);

    // Initialize OLED display
    println!("Initializing OLED display...");
    
    // Display initialization sequence
    let init_commands = [
        0xAE, // Display off
        0xD5, // Set display clock
        0x80, // [7:4] = 1000b (default), [3:0] = 0000b (default)
        0xA8, // Set multiplex ratio
        0x3F, // 1/64 duty
        0xD3, // Set display offset
        0x00, // No offset
        0x40, // Set display start line
        0x8D, // Charge pump
        0x14, // Enable charge pump
        0x20, // Set memory addressing mode
        0x00, // Horizontal addressing mode
        0xA1, // Set segment remap
        0xC8, // Set COM output direction
        0xDA, // Set COM pins
        0x12, // Alternative COM pin configuration
        0x81, // Set contrast
        0xCF, // Contrast value
        0xD9, // Set pre-charge period
        0xF1, // [7:4] = 1111b, [3:0] = 0001b
        0xDB, // Set VCOMH deselect level
        0x40, // 0.83 * Vcc
        0xA4, // Display from RAM
        0xA6, // Normal display (not inverted)
        0xAF, // Display on
    ];

    // Send initialization commands
    for cmd in init_commands.iter() {
        let control_byte = 0x00; // Control byte for commands
        let data = [control_byte, *cmd];
        match i2c.write(OLED_ADDR, &data, BLOCK) {
            Ok(_) => println!("Sent command: 0x{:02X}", cmd),
            Err(e) => {
                println!("Failed to send command 0x{:02X}: {:?}", cmd, e);
                return;
            }
        }
        FreeRtos::delay_ms(10);
    }

    // Clear the display
    println!("Clearing display...");
    let clear_commands = [
        0x21, // Set column address
        0x00, // Start column
        0x7F, // End column
        0x22, // Set page address
        0x00, // Start page
        0x07, // End page
    ];

    for cmd in clear_commands.iter() {
        let control_byte = 0x00;
        let data = [control_byte, *cmd];
        match i2c.write(OLED_ADDR, &data, BLOCK) {
            Ok(_) => println!("Sent command: 0x{:02X}", cmd),
            Err(e) => {
                println!("Failed to send command 0x{:02X}: {:?}", cmd, e);
                return;
            }
        }
        FreeRtos::delay_ms(10);
    }

    // Send some test data
    println!("Sending test pattern...");
    let control_byte = 0x40; // Control byte for data
    let mut data = [0u8; 129]; // 1 control byte + 128 data bytes
    data[0] = control_byte;
    
    // Create a simple pattern
    for i in 1..129 {
        data[i] = if (i % 2) == 0 { 0xAA } else { 0x55 };
    }

    match i2c.write(OLED_ADDR, &data, BLOCK) {
        Ok(_) => println!("Test pattern sent successfully"),
        Err(e) => {
            println!("Failed to send test pattern: {:?}", e);
            return;
        }
    }

    println!("OLED display test completed!");
} 