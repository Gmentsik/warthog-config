#![warn(clippy::pedantic)]
#![windows_subsystem = "windows"]

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

fn command_line_args<'a>() -> App<'a> {
    App::new(built_info::PKG_NAME)
        .version(built_info::PKG_VERSION)
        .author(built_info::PKG_AUTHORS)
        .about(built_info::PKG_DESCRIPTION)
        .arg(Arg::new("backlight")
            .short('b')
            .takes_value(true)
            .validator_regex(Regex::new("[0-1]").unwrap(), "must be 0 (off) or 1 (on)")
            .about("Turn the backlight on or off, default on, use 0 to turn it off or 1 to turn it on"))
        .arg(Arg::new("led-1")
            .short('1')
            .long("led-1")
            .takes_value(true)
            .validator_regex(Regex::new("[0-1]").unwrap(), "must be 0 (off) or 1 (on)")
            .about("Turn the first LED on or off, use 0 to turn it off or 1 to turn it on"))
        .arg(Arg::new("led-2")
            .short('2')
            .long("led-2")
            .takes_value(true)
            .validator_regex(Regex::new("[0-1]").unwrap(), "must be 0 (off) or 1 (on)")
            .about("Turn the second LED on or off, use 0 to turn it off or 1 to turn it on"))
        .arg(Arg::new("led-3")
            .short('3')
            .long("led-3")
            .takes_value(true)
            .validator_regex(Regex::new("[0-1]").unwrap(), "must be 0 (off) or 1 (on)")
            .about("Turn the third LED on or off, use 0 to turn it off or 1 to turn it on"))
        .arg(Arg::new("led-4")
            .short('4')
            .long("led-4")
            .takes_value(true)
            .validator_regex(Regex::new("[0-1]").unwrap(), "must be 0 (off) or 1 (on)")
            .about("Turn the fourth LED on or off, use 0 to turn it off or 1 to turn it on"))
        .arg(Arg::new("led-5")
            .short('5')
            .long("led-5")
            .takes_value(true)
            .validator_regex(Regex::new("[0-1]").unwrap(), "must be 0 (off) or 1 (on)")
            .about("Turn the fifth LED on or off, use 0 to turn it off or 1 to turn it on"))
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
        .arg(Arg::new("leds")
            .short('l')
            .long("leds")
            .takes_value(true)
            .validator_regex(Regex::new("[0-1]").unwrap(), "must be 0 (off) or 1 (on)")
            .about("Turn the all LEDs on or off, use 0 to turn it off or 1 to turn it on"))
}

fn main() -> Result<(), CustomError> {
    let matches = command_line_args().get_matches();

    let mut context = Context::new()
        .map_err(|err| CustomError(format!("can't create a USB context: {}", err)))?;

    // Open the USB device
    let (mut device, mut handle) = usb::open_device(
        &mut context,
        warthog::VID,
        warthog::THROTTLE_PID,
    ).expect("Failed to open the Warthog throttle. Is it connected?");

    println!(
        "Found Warthog throttle on endpoint {}.{}.{}",
        device.bus_number(),
        device.address(),
        device.port_number(),
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

    let (current_leds, current_intensity) = usb::read_warthog_throttle_config(&mut handle, readable_endpoint.address)
        .map_err(|err| {
            // Cleanup
            let _ = usb::release_usb_endpoint(&mut handle, readable_endpoint.iface, r_endpoint_has_kernel_driver);

            CustomError(format!("can't read the Warthog throttle configuration: {}", err))
        })?;

    println!("Current configuration:");
    println!("LEDs: {}", current_leds);
    println!("Intensity: {}", current_intensity);

    // Cleanup
    usb::release_usb_endpoint(&mut handle, readable_endpoint.iface, r_endpoint_has_kernel_driver)
        .map_err(|err| CustomError(format!("can't release the readable USB endpoint: {}", err)))?;

    if matches.is_present("read-only") {
        return Ok(())
    }

    let intensity: u8 = matches.value_of_t("intensity").unwrap();
    
    let mut leds = current_leds;

    if matches.value_of("leds") == Some("1") {
        leds = warthog::ThrottleLEDState::LED_ON;
    } else if matches.value_of("leds") == Some("0") {
        leds = warthog::ThrottleLEDState::LED_OFF;
    }


    if matches.value_of("backlight") == Some("1") {
        leds |= warthog::ThrottleLEDState::BACKLIGHT;
    } else if matches.value_of("backlight") == Some("0") {
        leds &= !warthog::ThrottleLEDState::BACKLIGHT;
    }

    if matches.value_of("led-1") == Some("1") {
        leds |= warthog::ThrottleLEDState::LED_1;
    } else if matches.value_of("led-1") == Some("0") {
        leds &= !warthog::ThrottleLEDState::LED_1;
    }

    if matches.value_of("led-2") == Some("1") {
        leds |= warthog::ThrottleLEDState::LED_2;
    } else if matches.value_of("led-2") == Some("0") {
        leds &= !warthog::ThrottleLEDState::LED_2;
    }  
    
    if matches.value_of("led-3") == Some("1") {
        leds |= warthog::ThrottleLEDState::LED_3;
    } else if matches.value_of("led-3") == Some("0") {
        leds &= !warthog::ThrottleLEDState::LED_3;
    } 
    
    if matches.value_of("led-4") == Some("1") {
        leds |= warthog::ThrottleLEDState::LED_4;
    } else if matches.value_of("led-4") == Some("0") {
        leds &= !warthog::ThrottleLEDState::LED_4;
    }

    if matches.value_of("led-5") == Some("1") {
        leds |= warthog::ThrottleLEDState::LED_5;
    } else if matches.value_of("led-5") == Some("0") {
        leds &= !warthog::ThrottleLEDState::LED_5;
    }


    println!();

    if current_leds == leds && current_intensity == intensity {
        println!("Nothing to update");
        return Ok(())
    }

    // Claim and configure the device
    usb::configure_endpoint(&mut handle, &writable_endpoint)
        .map_err(|err| CustomError(format!("can't configure readable endpoint: {}", err)))?;

    println!("Setting the Warthog throttle LEDs to {} and the intensity to {}...", leds, intensity);

    // Set the LEDs and intensity
    let wrote_size = usb::write_warthog_throttle_config(&mut handle, writable_endpoint.address, leds, intensity)
        .map_err(|err| {
            // Cleanup
            let _ = usb::release_usb_endpoint(&mut handle, writable_endpoint.iface, w_endpoint_has_kernel_driver);

            CustomError(format!("can't write the Warthog throttle configuration: {}", err))
        })?;

    if wrote_size != usb::WARTHOG_PACKET_DATA_LENGTH {
        // Cleanup
        let _ = usb::release_usb_endpoint(&mut handle, writable_endpoint.iface, w_endpoint_has_kernel_driver);

        return Err(CustomError(format!("should have written {} bytes but wrote {} bytes", usb::WARTHOG_PACKET_DATA_LENGTH, wrote_size)))
    }

    // Cleanup
    usb::release_usb_endpoint(&mut handle, writable_endpoint.iface, w_endpoint_has_kernel_driver)
        .map_err(|err| CustomError(format!("can't release the writable USB endpoint: {}", err)))?;

    println!("Done");

    Ok(())
}
