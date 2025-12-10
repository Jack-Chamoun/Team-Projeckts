#include <Wire.h>
#include <WiFi.h>
#include <Adafruit_Sensor.h>
#include <Adafruit_BMP280.h>
#include <ArduinoJson.h>
#include <WebServer.h>
#include <ESPmDNS.h>
#include "secrets.h"


// Create BMP object
Adafruit_BMP280 bmp; // I2C

// Init WebServer on port 80
WebServer server(80);

float temperature = 0.0;

void getTemperature() {
 
  StaticJsonDocument<200> jsonDoc;
  jsonDoc["temperature"] = temperature;
  String response;
  serializeJson(jsonDoc, response);
  // Set content type and send
  server.send(200, "application/json", response);

}

void setup() {
  // bauld rate
  Serial.begin(115200);

  // Setup of the Sensor
  Wire.begin(21, 22);
  Serial.println(F("BMP280 test"));
  
  if (!bmp.begin(0x76)) {
    Serial.println("Could not find a valid BMP280 sensor, check wiring!");
    while (1);
  }


  // Connect to Wifi
  Serial.print("Connecting to ");
  Serial.println(WIFI_SSID);
  WiFi.begin(WIFI_SSID, WIFI_PASSWORD);
  while (WiFi.status() != WL_CONNECTED) {
    delay(500);
    Serial.print(".");
  }

 // Start mDNS
    if (!MDNS.begin("roomtemperature")) {
        Serial.println("Error starting mDNS");
        return;
    }
    Serial.println("mDNS responder started");
  
  // Print local IP address and start web server
  Serial.println("");
  Serial.println("WiFi connected.");
  Serial.println("IP address: ");
  Serial.println(WiFi.localIP());

  // configure web server cors header and route handler
  server.enableCORS(true);
  server.on("/temperature", getTemperature);
  server.begin();
  Serial.println("HTTP server started and is available at: http://roomtemperature.local/temperature");
  
}

void loop() {
    temperature = bmp.readTemperature(); 
    server.handleClient();
    delay(2000); // pass control to cpu for 2 secs
}

