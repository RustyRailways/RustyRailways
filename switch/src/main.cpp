/**
 * @file main.cpp
 * @brief ESP8266 Rail Track Switch Control System
 * @author Michele Callegari
 * @date 11/02/2024
 */

#include <ESP8266WiFi.h>
#include <WiFiClient.h>
#include <ESP8266WebServer.h>
#include <Arduino.h>
#include <Servo.h>
#include <ArduinoJson.h>

#ifndef STASSID
    #define STASSID "Rusty Railways"
    #define STAPSK "rustyrailways"
#endif

#if S1
    #define STRAIGHT 180
    #define DIVERTED 20
    #define SWITCH_NAME "s1"
#endif

#if S2
   #define STRAIGHT 170
    #define DIVERTED 10
    #define SWITCH_NAME "s2"
#endif

#if S3
    #define STRAIGHT 180
    #define DIVERTED 0
    #define SWITCH_NAME "s3"
#endif

#if S4
   #define STRAIGHT 175
    #define DIVERTED 5
    #define SWITCH_NAME "s4"
#endif

#if S5
    #define STRAIGHT 170
    #define DIVERTED 15
    #define SWITCH_NAME "s5"
#endif

#if S6
    #define STRAIGHT 175
    #define DIVERTED 5
    #define SWITCH_NAME "s6"
#endif

#ifndef STRAIGHT
    #define STRAIGHT 180
#endif

#ifndef DIVERTED
    #define DIVERTED 20
#endif

#ifndef SWITCH_NAME
    #define SWITCH_NAME "general switch"
#endif

#define JSON_DIM 256 // Adjust the size based on your JSON message, default 256

const char* host = SWITCH_NAME;
const char* ssid = STASSID;
const char* password = STAPSK;

ESP8266WebServer server(80);
Servo servo;

/**
 * @brief Handles the root endpoint, sending hardware and switch information in JSON format.
 */
void handleRoot() {
    DynamicJsonDocument doc(JSON_DIM);
    doc["type"] = host;
    doc["hardwareId"] = ESP.getChipId();
    String message;
    serializeJson(doc, message);
    server.send(200, "text/plain", message);
}

/**
 * @brief Opens the switch, setting it to the straight position.
 */
void openSwitch() {
    String response = "switch open";
    servo.write(STRAIGHT);
    server.send(200, "text/plain", response);
}

/**
 * @brief Closes the switch, setting it to the diverted position.
 */
void closeSwitch() {
    String response = "switch close";
    servo.write(DIVERTED);
    server.send(200, "text/plain", response);
}

/**
 * @brief Handles HTTP POST requests to set the switch position.
 */
void handlerPost() {
    String rawContent = server.arg("plain");
    if (rawContent == "\"SetPositionStraight\"") {
        openSwitch();
    } else if (rawContent == "\"SetPositionDiverted\"") {
        closeSwitch();
    } else {
        String response = "request not found";
        server.send(404, "text/plain", response);
    }
}

/**
 * @brief Setup function, initializes the system and starts the HTTP server.
 */
void setup() {
    servo.attach(2, 500, 2400);
    servo.write(STRAIGHT);
    delay(100);
    WiFi.mode(WIFI_AP_STA);
    WiFi.begin(ssid, password);
    if (WiFi.waitForConnectResult() == WL_CONNECTED) {
        // Define server routes
        server.on("/", HTTP_POST, handlerPost);
        server.on("/", HTTP_GET, handleRoot);

        // Start server
        server.begin();
    } else {
        //Serial.println("WiFi Failed");
    }
}

/**
 * @brief Loop function, handles incoming client requests.
 */
void loop() {
    server.handleClient();
}
