#include <WiFi.h>

const char* ssid = "ESP32-Test";
const char* password = "1234";

WiFiServer server(80);

void setup() {
  Serial.begin(115200);
  Serial.println("\n[*] Creating Access Point");
  WiFi.mode(WIFI_AP);
  WiFi.softAP(ssid, password);
  Serial.print("[+] Access Point created with IP gateway: ");
  Serial.println(WiFi.softAPIP());
  
  server.begin();
}

void display_webpage(WiFiClient client){
  // Send header
  client.println("HTTP/1.1 200 OK");
  client.println("Content-type:text/html");
  client.println("Connection: close");
  client.println();

  // Send HTML head
  client.println("<!DOCTYPE html><html>");
  client.println("<head><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">");
  client.println("<link rel=\"icon\" href=\"data:,\">");
  client.println("<style>html { font-family: Helvetica; display: inline-block; margin: 0px auto; text-align: center;}");
  client.println(".button { background-color: #4CAF50; border: none; color: white; padding: 16px 40px;");
  client.println("text-decoration: none; font-size: 30px; margin: 2px; cursor: pointer;}");
  client.println(".button2 {background-color: #555555;}</style></head>");

  // Send HTML body
  client.println("<body><h1>Hello</h1></body></html>");
  
  // Send blank line to signify the end of message
  client.println();
}

void loop(){
  WiFiClient client = server.available(); // Listen

  if (client) {
    Serial.println("Client Connected");
    String currentLine = "";
    String header = "";
    while (client.connected()){
      if (client.available()){
        char c = client.read();
        Serial.write(c);
        header += c;
        if (c == '\n'){
          if (currentLine.length() == 0){
            display_webpage(client);
            break;
          }
          else {
            // Clear the current line if a newline is returned \n\n
            currentLine = "";
          }
        } 
        else if (c != '\r') {
          // Add character to current line if its not a return char
          currentLine += c;
        }
      }
    }
    
    // Reset the header
    header = "";
    // Stop communications with client
    client.stop();
    Serial.println("Client disconnected.\n");
  }
  delay(10);
}
