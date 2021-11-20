#include <SPIFFS.h>
#include <WiFi.h>
#include "ESPAsyncWebServer.h"

const char* ssid = "ESP32-Test";
const char* password = "1234";

AsyncWebServer server(80);

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

void display_webpage(WiFiClient client){
  // Send header
  client.println("HTTP/1.1 200 OK");
  client.println("Content-type:text/html");
  client.println("Connection: close");
  client.println();
  
  // Send blank line to signify the end of message
  client.println();
}

void loop(){
//  WiFiClient client = server.available(); // Listen
//
//  if (client) {
//    Serial.println("Client Connected");
//    String currentLine = "";
//    String header = "";
//    while (client.connected()){
//      if (client.available()){
//        char c = client.read();
//        Serial.write(c);
//        header += c;
//        if (c == '\n'){
//          if (currentLine.length() == 0){
//            display_webpage(client);
//            break;
//          }
//          else {
//            // Clear the current line if a newline is returned \n\n
//            currentLine = "";
//          }
//        } 
//        else if (c != '\r') {
//          // Add character to current line if its not a return char
//          currentLine += c;
//        }
//      }
//    }
//    
//    // Reset the header
//    header = "";
//    // Stop communications with client
//    client.stop();
//    Serial.println("Client disconnected.\n");
//  }
//  delay(10);
}
