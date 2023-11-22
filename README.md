# Thrustmaster HOTAS Warthog™ Configuration Tool

![HOTAS Warthog](assets/warthog_hotas.png)

[![build](https://github.com/Gmentsik/warthog-config/workflows/build/badge.svg)](https://github.com/Gmentsik/warthog-config/actions?query=workflow%3Abuild)

Command-line tool that can be used to control the LEDs of a Thrustmaster HOTAS Warthog throttle.

## Download

<https://github.com/Gmentsik/warthog-config/releases>

## Usage

```text
$ ./warthog-config --help
Command-line tool that can be used to control the LEDs of a Thrustmaster HOTAS Warthog throttle.

USAGE:
    warthog-config [FLAGS] [OPTIONS]

FLAGS:
    -h, --help         Print help information
    -r, --read-only    Only show the current state, don't change the LEDs
    -V, --version      Print version information

OPTIONS:
    -1, --led-1 <led-1>            Turn the first LED on or off, use 0 to turn it off or 1 to turn it on
    -2, --led-2 <led-2>            Turn the second LED on or off, use 0 to turn it off or 1 to turn it on
    -3, --led-3 <led-3>            Turn the third LED on or off, use 0 to turn it off or 1 to turn it on
    -4, --led-4 <led-4>            Turn the fourth LED on or off, use 0 to turn it off or 1 to turn it on
    -5, --led-5 <led-5>            Turn the fifth LED on or off, use 0 to turn it off or 1 to turn it on
    -b <backlight>                 Turn the backlight on or off, default on, use 0 to turn it off or 1 to turn it on
    -i, --intensity <intensity>    Set the intensity of the backlight (0-5, where 0 in off and 5 is the brightest) [default: 2]
    -l, --leds <leds>              Turn the all LEDs on or off, use 0 to turn it off or 1 to turn it on
```

### Examples

* Disable all the LEDs

    ```sh
    ./warthog-config -l 0
    ```
* Disable all the LEDs enable backlight 

    ```sh
    ./warthog-config -b 1 -l 0
    ```

* Disable all the LEDs and backlight 

    ```sh
    ./warthog-config -b 0 -l 0
    ```

* Set LED 4 ON, leaving rest as-is

    ```sh
    ./warthog-config -4 1
    ```

* Set LED 5 ON LED 1 ON LED 3 OFF, leaving rest as-is

    ```sh
    ./warthog-config -1 1 -5 1 -3 0
    ```

* Set backlight intensity to max

    ```sh
    ./warthog-config -i 5
    ```

# How to use with Microsoft Flight Simulator 

1. Download & Install SPAD.neXt (https://www.spadnext.com/download/download-spad-next.html)
2. Go to Settings -> Devices -> Scriptpanel and turn on Script-Panel Support
3. A new icon should appear between profiles and devices in the left-hand vertical menue
4. Click on Add Event -> Conditional Action
5. Here you can define the contion (eg. AUTOPILOT MASTER equals 1)
6. As action add "Advanced -> Run external Program"
7. Point to the exe you downloaded here from the releases page in the field "Executable" (eg "G:\warthog-config.exe")
8. In the command-line arguments field you can write anything, check the examples above. (eg "-5 1" to turn LED 5 on)
