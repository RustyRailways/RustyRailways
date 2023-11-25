#include <ESP8266WiFi.h>
#include <ESP8266WebServer.h>
#include <Arduino.h>
//#include <Servo.h>

// Replace with your Wi-Fi credentials
const char* ssid = "wifi-ssid";
const char* password = "wifi-password";

ESP8266WebServer server(80);

//Servo servo;

// Handle client requests
void handleRoot() {
  server.send(200, "text/plain", "server root,\n ESP8266");
}

void handleTest() {
  String response = "test";
  Serial.println("test");
  server.send(200, "text/plain", response);
}

void handleOpenSwitch() {
  String response = "switch open";
  Serial.println("switch open");
  // servo.write(90);
  server.send(200, "text/plain", response);
}

void handleCloseWitch() {
  String response = "switch close";
  Serial.println("switch close");
  // servo.write(0);
  server.send(200, "text/plain", response);
}

void setup() {
  Serial.begin(115200);
  //servo.attach(D1, 500, 2400);
  // Connect to Wi-Fi
  WiFi.begin(ssid, password);
  while (WiFi.status() != WL_CONNECTED) {
    delay(1000);
    Serial.println("Connecting to WiFi...");
  }
  Serial.println("Connected to WiFi");

  // Define server routes
  server.on("/", HTTP_GET, handleRoot);
  server.on("/test", HTTP_GET, handleTest);
  server.on("/api/v1/open", HTTP_GET, handleOpenSwitch);
  server.on("/api/v1/close", HTTP_GET, handleCloseWitch);

  // Start server
  server.begin();
  Serial.println("HTTP server started");
}

void loop() {
  server.handleClient();
}