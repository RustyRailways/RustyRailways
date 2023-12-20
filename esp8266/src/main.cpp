#include <ESP8266WiFi.h>
#include <WebSocketsServer.h>
#include <ArduinoJson.h>
#include <Servo.h>

#define JSON_DIM 256 // Adjust the size based on your JSON message, default 256
#define HEART_BEAT_INTERVAL 1000 // 1 sec

const char *ssid = "your-ssid";
const char *password = "your-password";

unsigned long lastHeartbeatTime = 0;
WebSocketsServer webSocket = WebSocketsServer(81);
Servo servo;
bool open;

void sendStatus() {
  DynamicJsonDocument doc(JSON_DIM);
  doc["HeartBeatFrom"] = ESP.getChipId();
  doc["Status"] = open ? "open" : "close";
  String heartbeatMessage;
  serializeJson(doc, heartbeatMessage);
  webSocket.broadcastTXT(heartbeatMessage);
}

void OpenSwitch() {
  String response = "switch open";
  Serial.println("switch open");
  servo.write(90);
  open = true;
}

void CloseWitch() {
  String response = "switch close";
  Serial.println("switch close");
  servo.write(0);
  open = false;
}

void handleWebSocketMessage(uint8_t num, WStype_t type, uint8_t *payload, size_t length) {
  if (type == WStype_TEXT) {
    // Parse JSON message
    DynamicJsonDocument doc(JSON_DIM);
    deserializeJson(doc, payload);

    // Check for the "action" field in the JSON
    const char *action = doc["action"];

    // Perform action based on JSON content
    if (strcmp(action, "open") == 0) {
      OpenSwitch();
    }
    if (strcmp(action, "close") == 0) {
      CloseWitch();
    }
    if (strcmp(action, "status") == 0) {
      sendStatus();
    }
  }
}

// send JSON heartbeat message
void sendHeartbeat() {
  DynamicJsonDocument doc(JSON_DIM);
  doc["HeartBeatFrom"] = ESP.getChipId();
  String heartbeatMessage;
  serializeJson(doc, heartbeatMessage);
  webSocket.broadcastTXT(heartbeatMessage);
}

void setup() {
  Serial.begin(115200);
  servo.attach(2, 500, 2400);
  OpenSwitch();
  Serial.begin(115200);
  WiFi.begin(ssid, password);

  while (WiFi.status() != WL_CONNECTED) {
    delay(250);
    Serial.print(".");
  }

  Serial.println("");
  Serial.println("WiFi connected");
  Serial.println("IP address: ");
  Serial.println(WiFi.localIP());

  // Begin WebSocket server
  webSocket.begin();
  // Set the event handler
  webSocket.onEvent(handleWebSocketMessage);
}

void loop() {
  // Handle WebSocket events
  webSocket.loop();

  // Heartbeat
  unsigned long currentMillis = millis();
  if (currentMillis - lastHeartbeatTime >= HEART_BEAT_INTERVAL) {
    sendHeartbeat();
    lastHeartbeatTime = currentMillis;
  }
}
