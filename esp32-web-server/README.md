# Getting started

## Installation

- Install Arduino CLI

` https://arduino.github.io/arduino-cli/1.2/installation/ `

## Preparing the ESP32 Controller

You need:
- Mini D1 ESP32
- BME280 Sensor Module
- 4 wires
- USB Cable

Wiring:
- Place the controller on the edge of the breadboard
- Connect the wires as following: 
**BME280 Sensor Module -> Mini D1 ESP32:**
    - VCC -> 3.3V
    - GNC -> GNC
    - SCL -> IO22
    - SDA -> IO21
- Connect the controller to the computer

## Setup

1. Open the Arduino CLI

2. Go to File > Preferences > Click on the Icon next to "Additional boards manager URLS:"  > Enter the URL: https://espressif.github.io/arduino-esp32/package_esp32_index.json > Click "Ok" > Go to Tools > Boards Manager > Search for "esp32"  and install it (not esp32 boards)
Source: https://docs.espressif.com/projects/arduino-esp32/en/latest/installing.html#installing-using-boards-manager

3. Got to Tools > Boards > Boards Manager > Search for "Wemos D1 Mini Esp32" and select your USB Port (for me it was "COM3")

4. Go to Tools > Manage Libraries > Install these libraries: 
- Adafruit BMP280
- Adafruit BusIO
- Adafruit Unified Sensor
- ArduinoJson


5. Create a secrets.h  file like in secrets.h.example with your ssid and  wifi password. And also in temperatues.ino set the ssid to your routers ssid.

6. Create a temperature.ino file and paste the contents of the current temperature.ino file (on Gitlab) in this file

7. If your client runs on WSL/Virtual Machine in Ubuntu:
    - The name resolution via dns doesnt work there, you have to either give the ip address or install a mdns responder with these libraries to enable dns-resolution from host:
    ```
    sudo apt update
    sudo apt install avahi-daemon libnss-mdns
    # restart Wsl or reload /etc/nsswitch.conf
    # /etc/nsswitch.conf should have this line:
    # hosts: files mdns4_minimal [NOTFOUND=return] dns

    ```


## Run the web server

### Server

1. Upload the code 
2. Go to http://roomtemperature.local/temperature. You should see a json of the current room temperature like this: {"temperature":25.49}

### Client

1. Leptos Client should just run: 
```trunk serve [--open]```


