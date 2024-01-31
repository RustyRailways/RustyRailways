/*
  To upload through terminal you can use: curl -F "image=@firmware.bin" esp8266-webupdate.local/update
*/

#include <ESP8266WiFi.h>
#include <WiFiClient.h>
#include <ESP8266WebServer.h>
#include <ESP8266mDNS.h>
#include <Arduino.h>
#include <Servo.h>
#include <ArduinoJson.h>
#include <ArduinoOTA.h>

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
ESP8266WebServer serverUpdate(8266);
const char* serverIndex = "<form method='POST' action='/update' enctype='multipart/form-data'><input type='file' name='update'><input type='submit' value='Update'></form>";


Servo servo;

// Handle client requests
void handleRoot() {
    DynamicJsonDocument doc(JSON_DIM);
    doc["type"] = host;
    doc["hardwareId"] = ESP.getChipId();
    String message;
    serializeJson(doc, message);
    server.send(200, "text/plain", message);
}

void  openSwitch() {
    String response = "switch open";
    //Serial.println("switch open");
    servo.write(STRAIGHT);
    server.send(200, "text/plain", response);
}

void closeSwitch() {
    String response = "switch close";
    //Serial.println("switch close");
    servo.write(DIVERTED);
    server.send(200, "text/plain", response);
}

void handlerPost(){
    //Seri"SetPositionStraight"al.println("POST request arrived");
    String rawContent = server.arg("plain");
    if(rawContent == "\"SetPositionStraight\""){
        openSwitch();
    }else if(rawContent == "\"SetPositionDiverted\""){
        closeSwitch();
    }else{
        String response = "request not found";
        server.send(404, "text/plain", response);
    }
  
}


void setup() {
    //Serial.begin(115200);
    servo.attach(2, 500, 2400);
    servo.write(STRAIGHT);
    delay(100);
    WiFi.mode(WIFI_AP_STA);
    WiFi.begin(ssid, password);
    if (WiFi.waitForConnectResult() == WL_CONNECTED) {
        MDNS.begin(host);

        // Define server routes
        server.on("/", HTTP_POST, handlerPost);
        server.on("/", HTTP_GET, handleRoot);

        // ota update
        serverUpdate.on("/", HTTP_GET, []() {
            serverUpdate.sendHeader("Connection", "close");
                serverUpdate.send(200, "text/html", serverIndex);
        });
        serverUpdate.on("/update", HTTP_POST, []() {
            serverUpdate.sendHeader("Connection", "close");
            serverUpdate.send(200, "text/plain", (Update.hasError()) ? "FAIL" : "OK");
            ESP.restart();
        },[]() { HTTPUpload& upload = serverUpdate.upload();
                if (upload.status == UPLOAD_FILE_START) {
                //Serial.setDebugOutput(true);
                WiFiUDP::stopAll();
                //Serial.printf("Update: %s\n", upload.filename.c_str());
                uint32_t maxSketchSpace = (ESP.getFreeSketchSpace() - 0x1000) & 0xFFFFF000;
                if (!Update.begin(maxSketchSpace)) {  // start with max available size
                    //Update.printError(Serial);
                }
                } else if (upload.status == UPLOAD_FILE_WRITE) {
                if (Update.write(upload.buf, upload.currentSize) != upload.currentSize) {
                    //Update.printError(Serial);
                }
                } else if (upload.status == UPLOAD_FILE_END) {
                if (Update.end(true)) {  // true to set the size to the current progress
                    //Serial.printf("Update Success: %u\nRebooting...\n", upload.totalSize);
                } else {
                    Update.printError(Serial);
                }
                //Serial.setDebugOutput(false);
                }
                yield();
        });
                
        // Start server
        server.begin();
        MDNS.addService("http", "tcp", 80);
        serverUpdate.begin();
        MDNS.addService("http", "tcp", 8266);
        //Serial.printf("Ready! Open http://%s.local:8266 in your browser\n", host);
        //Serial.println("HTTP server started");
    } else {
        //Serial.println("WiFi Failed");
    }
}

void loop() {
    server.handleClient();
    serverUpdate.handleClient();  
    MDNS.update();
}
