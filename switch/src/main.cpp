//old version using rest API

#include <ESP8266WiFi.h>
#include <ESP8266WebServer.h>
#include <Arduino.h>
#include <Servo.h>
#include <ArduinoJson.h>

#define JSON_DIM 256 // Adjust the size based on your JSON message, default 256

// Replace with your Wi-Fi credentials
//const char *ssid = "your-ssid";
//const char *password = "your-password";

const char *ssid = "Rete abc";
const char *password = "ciaociaocattai";


ESP8266WebServer server(80);
Servo servo;

// Handle client requests
void handleRoot() {
  DynamicJsonDocument doc(JSON_DIM);
  doc["type"] = "switch";
  doc["hardwareId"] = ESP.getChipId();
  String message;
  serializeJson(doc, message);
  server.send(200, "text/plain", message);
}

void  openSwitch() {
  String response = "switch open";
  Serial.println("switch open");
  servo.write(90);
  server.send(200, "text/plain", response);
}

void closeSwitch() {
  String response = "switch close";
  Serial.println("switch close");
  servo.write(0);
  server.send(200, "text/plain", response);
}

void handlerPost(){
  Serial.println("POST request arrived");
  String rawContent = server.arg("plain");
  if(rawContent == "\"SetPositionStraight\""){
    openSwitch();
  }else if(rawContent == "\"SetPositionDiverging\""){
    closeSwitch();
  }else{
    String response = "request not found";
    server.send(404, "text/plain", response);
  }
  
}

void setup() {
  Serial.begin(115200);
  servo.attach(2, 500, 2400);
  servo.write(90);
  delay(100);
  // Connect to Wi-Fi
  WiFi.begin(ssid, password);
  while (WiFi.status() != WL_CONNECTED) {
    delay(1000);
    Serial.println("Connecting to WiFi...");
  }
  Serial.println("Connected to WiFi");

  // Define server routes
  server.on("/", HTTP_POST, handlerPost);
  server.on("/", HTTP_GET, handleRoot);

  // Start server
  server.begin();
  Serial.println("HTTP server started");
}

void loop() {
  server.handleClient();
}