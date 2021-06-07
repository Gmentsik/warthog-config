use std::fmt;
use clap::{Arg, App};
use regex::Regex;
use rusb::Context;

mod usb;
mod warthog;

pub mod built_info {
   // The file has been placed there by the build script.
   include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

struct CustomError(String);

impl fmt::Debug for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() -> Result<(), CustomError> {
    let matches = App::new(built_info::PKG_NAME)
        .version(built_info::PKG_VERSION)
        .author(built_info::PKG_AUTHORS)
        .about(built_info::PKG_DESCRIPTION)
        .arg(Arg::new("backlight")
            .short('b')
            .long("backlight")
            .about("Turn the backlight on"))
        .arg(Arg::new("led-1")
            .short('1')
            .long("led-1")
            .about("Turn the first LED on"))
        .arg(Arg::new("led-2")
            .short('2')
            .long("led-2")
            .about("Turn the second LED on"))
        .arg(Arg::new("led-3")
            .short('3')
            .long("led-3")
            .about("Turn the third LED on"))
        .arg(Arg::new("led-4")
            .short('4')
            .long("led-4")
            .about("Turn the fourth LED on"))
        .arg(Arg::new("led-5")
            .short('5')
            .long("led-5")
            .about("Turn the fifth LED on"))
        .arg(Arg::new("intensity")
            .short('i')
            .long("intensity")
            .validator_regex(Regex::new("[0-5]").unwrap(), "must be between 0 and 5")
            .takes_value(true)
            .default_value("2")
            .about("Set the intensity of the backlight (0-5, where 0 in off and 5 is the brightest)"))
        .arg(Arg::new("read-only")
            .short('r')
            .long("read-only")
            .about("Only show the current state, don't change the LEDs"))
        .get_matches();

    let mut context = Context::new()
        .map_err(|err| CustomError(format!("can't create a USB context: {}", err)))?;

    // Open the USB device
    let (mut device, mut handle) = usb::open_device(
        &mut context,
        warthog::VID,
        warthog::THROTTLE_PID
    ).expect("Failed to open the Warthog throttle. Is it connected?");

    println!(
        "Found Warthog throttle on bus {}.{}.{}",
        device.bus_number(),
        device.address(),
        device.port_number()
    );

    if cfg!(debug_assertions) {
        usb::print_device_info(&mut handle)
            .map_err(|err| CustomError(format!("can't print the device info: {}", err)))?;
    }

    // Get the USB endpoints for reading and writing
    let (readable_endpoints, writable_endpoints) = usb::find_endpoints(&mut device)
        .map_err(|err| CustomError(format!("can't find USB endpoints: {}", err)))?;
    let readable_endpoint = readable_endpoints
        .first()
        .expect("No readable endpoint found on device");
    let r_endpoint_has_kernel_driver = match handle.kernel_driver_active(readable_endpoint.iface) {
        Ok(true) => {
            handle.detach_kernel_driver(readable_endpoint.iface)
                .map_err(|err| CustomError(format!("can't detach kernel driver for interface {}: {}", readable_endpoint.iface, err)))?;
            true
        }
        _ => false,
    };
    let writable_endpoint = writable_endpoints
        .first()
        .expect("No readable endpoint found on device");
    let w_endpoint_has_kernel_driver = match handle.kernel_driver_active(writable_endpoint.iface) {
        Ok(true) => {
            handle.detach_kernel_driver(writable_endpoint.iface)
                .map_err(|err| CustomError(format!("can't detach kernel driver for interface {}: {}", writable_endpoint.iface, err)))?;
            true
        }
        _ => false,
    };

    // Claim and configure the device
    usb::configure_endpoint(&mut handle, &readable_endpoint)
        .map_err(|err| CustomError(format!("can't configure readable endpoint: {}", err)))?;

    let data = usb::read_interrupt(&mut handle, readable_endpoint.address)
        .map_err(|err| CustomError(format!("can't read interrupt: {}", err)))?;
    println!("{:02X?}", data);

    println!("Current configuration:");
    usb::print_data(data);

    // Cleanup
    handle.release_interface(readable_endpoint.iface)
        .map_err(|err| CustomError(format!("can't release the readable interface: {}", err)))?;
    if r_endpoint_has_kernel_driver {
        handle.attach_kernel_driver(readable_endpoint.iface)
            .map_err(|err| CustomError(format!("can't attach the kernel driver on the readable interface: {}", err)))?;
    }

    if matches.is_present("read-only") {
        return Ok(())
    }

    // Claim and configure the device
    usb::configure_endpoint(&mut handle, &writable_endpoint)
        .map_err(|err| CustomError(format!("can't configure readable endpoint: {}", err)))?;

    let intensity: u8 = matches.value_of_t("intensity").unwrap();
    let mut leds = warthog::ThrottleLEDState::empty();

    if matches.is_present("backlight") {
        leds |= warthog::ThrottleLEDState::BACKLIGHT;
    }
    if matches.is_present("led-1") {
        leds |= warthog::ThrottleLEDState::LED_1;
    }
    if matches.is_present("led-2") {
        leds |= warthog::ThrottleLEDState::LED_2;
    }
    if matches.is_present("led-3") {
        leds |= warthog::ThrottleLEDState::LED_3;
    }
    if matches.is_present("led-4") {
        leds |= warthog::ThrottleLEDState::LED_4;
    }
    if matches.is_present("led-5") {
        leds |= warthog::ThrottleLEDState::LED_5;
    }

    // Set the LEDs and intensity
    usb::write_interrupt(&mut handle, writable_endpoint.address, leds, intensity)
        .map_err(|err| CustomError(format!("can't write interrupt: {}", err)))?;

    // Cleanup
    handle.release_interface(writable_endpoint.iface)
        .map_err(|err| CustomError(format!("can't release the writable interface: {}", err)))?;
    if w_endpoint_has_kernel_driver {
        handle.attach_kernel_driver(writable_endpoint.iface)
            .map_err(|err| CustomError(format!("can't attach the kernel driver on the writable interface: {}", err)))?;
    }

    Ok(())
}
