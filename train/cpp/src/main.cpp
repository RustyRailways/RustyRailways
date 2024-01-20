#include <Arduino.h>
#include <WebSocketsServer.h>
#include <ArduinoJson.h>
#include <WiFi.h>
#include <Adafruit_PN532.h>

// motor pin
#define MOTOR_PIN1 19
#define MOTOR_PIN2 18
#define ENABLE_MOTOR 21

// nfc reader pin
#define SDA_PIN 22
#define SCL_PIN 23

// Adjust the size based on your JSON message, default 256
#define JSON_DIM 256 

// Replace with your Wi-Fi credentials
const char* ssid = "Rete abc";
const char* password = "ciaociaocattai";

// train control
enum speed_level{Slow, Mid_Slow, Moderate, Mid_Fast, Fast};
enum direction{Stop, Forward, Backward};

// websocket
WebSocketsServer webSocket = WebSocketsServer(80);

// nfc class costructor
Adafruit_PN532 nfc(SDA_PIN, SCL_PIN);

// Setting PWM properties
const int freq = 50;
const int pwmChannel = 0;
const int resolution = 8;
int dutyCycle = 200;

// last passed tag
int last_position;


void motor_setUp(){
    pinMode(MOTOR_PIN1, OUTPUT);
    pinMode(MOTOR_PIN2, OUTPUT);
    pinMode(ENABLE_MOTOR, OUTPUT);
    
    // configure LED PWM functionalitites
    ledcSetup(pwmChannel, freq, resolution);
    // attach the channel to the GPIO to be controlled
    ledcAttachPin(ENABLE_MOTOR, pwmChannel);
}

void wifi_setUp(){
    // Connect to Wi-Fi
    WiFi.begin(ssid, password);
    while (WiFi.status() != WL_CONNECTED) {
        delay(1000);
        Serial.println("Connecting to WiFi...");
    }
    Serial.println("Connected to WiFi");
}

void nfc_setUp(){
    nfc.begin();
    uint32_t versiondata = nfc.getFirmwareVersion();
    if (!versiondata) {
        Serial.print("Didn't find PN53x board");
        while (1);
    }
    // configure board to read RFID cards
    nfc.SAMConfig();
    Serial.println("Waiting for an NFC card ...");
}
   
void set_speed(int speed){
    switch (speed){
        case Slow:
            dutyCycle= 50;
            ledcWrite(pwmChannel, dutyCycle);
            break;
        case Mid_Slow:
            dutyCycle= 100;
            ledcWrite(pwmChannel, dutyCycle);
            break;
        case Moderate:
            dutyCycle= 150;
            ledcWrite(pwmChannel, dutyCycle);
            break;
        case Mid_Fast:
            dutyCycle= 200;
            ledcWrite(pwmChannel, dutyCycle);
            break;
        case Fast:
            dutyCycle= 250;
            ledcWrite(pwmChannel, dutyCycle);
            break;
        default:
            Serial.println("default speed");
            dutyCycle= 150;
            ledcWrite(pwmChannel, dutyCycle);
            break;
    }
    
}

void set_direction(direction dir){
    switch (dir){
        case Stop:
            digitalWrite(MOTOR_PIN1, LOW);
            digitalWrite(MOTOR_PIN2, LOW);
            break;
        case Forward:
            digitalWrite(MOTOR_PIN1, HIGH);
            digitalWrite(MOTOR_PIN2, LOW);
            break;
        case Backward:
            digitalWrite(MOTOR_PIN1, LOW);
            digitalWrite(MOTOR_PIN2, HIGH);
            break;
        default:
            Serial.println("default stop");
            digitalWrite(MOTOR_PIN1, LOW);
            digitalWrite(MOTOR_PIN2, LOW);
            break;
    }
}

void send_last_read_position(){
    DynamicJsonDocument doc(JSON_DIM);
    doc["last_read_position"] = String(last_position);
    String message;
    Serial.println("Send message: " + message);
    serializeJson(doc, message);
    webSocket.broadcastTXT(message);
}

void handleWebSocketMessage(String message) {
    Serial.println("Received WebSocket message: " + message);
    
    // Parse JSON message
    DynamicJsonDocument doc(JSON_DIM);
    deserializeJson(doc, message);
    const String action = doc["action"];
    const int value = doc["value"];
    Serial.println("action: " + action+" value: "+value);
    if( action != ""){
        
        // Perform action based on JSON content
        if(action=="stop"){
            set_direction(Stop);
        }
        if(action=="setSpeed"){
            Serial.println("setting speed");
            if(value==0){
                Serial.println("stop");
                set_direction(Stop);
            }else if(value>0){
                Serial.println("forward");
                set_direction(Forward);
                set_speed(value);
            }else{
                Serial.println("backward");
                set_direction(Backward);
                set_speed(value*-1);
            }
            
        }
        if(action=="readPosition"){
            send_last_read_position();
        }
    }
}

void onWebSocketEvent(uint8_t num, WStype_t type, uint8_t * payload, size_t length) {
    switch (type) {
        case WStype_TEXT:
            handleWebSocketMessage(String((char*)payload));
            break;
        case WStype_DISCONNECTED:
            Serial.printf("Disconnected!\n");
            set_speed(Slow);
            set_direction(Stop);
            break;
    }
}

void read_nfc(){
    uint8_t success;
    uint8_t uid[] = { 0, 0, 0, 0, 0, 0, 0 };
    uint8_t uidLength;
    success = nfc.readPassiveTargetID(PN532_MIFARE_ISO14443A, uid, &uidLength);
    if (success) {
        Serial.println("Found an NFC card!");

        Serial.print("UID Length: ");Serial.print(uidLength, DEC);Serial.println(" bytes");
        Serial.print("UID Value: ");
        for (uint8_t i=0; i < uidLength; i++) {
            Serial.print(" 0x");Serial.print(uid[i], HEX);
        }
        Serial.println("");   
    }
}

void setup() {
    
    Serial.begin(115200);

    motor_setUp();
    wifi_setUp();
    last_position=0;
    //nfc_setUp();
    
    // Begin WebSocket server
    webSocket.begin();
    // Set the event handler
    webSocket.onEvent(onWebSocketEvent);

    Serial.println("Train is started");
    
}

void loop() {
    webSocket.loop();
    //read_nfc();
}