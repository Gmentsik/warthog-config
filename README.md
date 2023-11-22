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

