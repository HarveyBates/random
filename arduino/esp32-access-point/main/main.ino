#include <SPIFFS.h>
#include <WiFi.h>

const char* ssid = "ESP32-Test";
const char* password = "1234";

WiFiServer server(80);

void setup() {
  Serial.begin(115200);

  // Setup SPI flash file system (SPIFFS)
  if(!SPIFFS.begin(true)){
    Serial.println("An Error has occurred while mounting SPIFFS");
    return;
  }

  // Setup access point
  Serial.println("\n[*] Creating Access Point");
  WiFi.mode(WIFI_AP);
  WiFi.softAP(ssid, password);
  Serial.print("[+] Access Point created with IP gateway: ");
  Serial.println(WiFi.softAPIP());
  

  // Setup requests
  server.on("/html", HTTP_GET, [](AsyncWebServerRequest *request){
    request->send(SPIFFS, "/test.html", "text/html");
  });

  // Start server
  server.begin();
}

void loop(){}
