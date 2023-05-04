#include <Arduino_JSON.h>
#include <ESP8266WiFi.h>
#include <ESP8266HTTPClient.h>

#define FASTLED_ESP8266_RAW_PIN_ORDER
#include <FastLED.h>

#include "config.h"

CRGBArray<NB_PIXELS> pixels;

void setup() {
    Serial.begin(115200);
    FastLED.addLeds<NEOPIXEL, PIXELS_PIN>(pixels, NB_PIXELS)
        .setCorrection(TypicalLEDStrip);
    setup_wifi();
}

void setup_wifi() {
    int led = 0;
    delay(10);

    Serial.printf("\nConnecting to %s\n", WIFI_SSID);

#ifdef WIFI_PSK
    WiFi.begin(WIFI_SSID, WIFI_PSK);
#else
    WiFi.begin(WIFI_SSID);
#endif

    while (WiFi.status() != WL_CONNECTED) {
        FastLED.clear();
        pixels[led] = color(led);
        FastLED.show();
        led = (led + 1) % NB_PIXELS;

        delay(500);
        Serial.print(".");
    }

    Serial.println("\nWiFi connected");
    Serial.printf("IP address: %s\n", WiFi.localIP());

    FastLED.clear();
    FastLED.show();
}

void loop() {
    JSONVar json = request();

    uint8_t pos = position(json);
    Serial.printf("%hu/%hu (%hu)\n", (uint16_t)json["funded"], (uint16_t)json["objective"], pos);

    gradient(pos);
    FastLED.show();

    delay(1000);
}

JSONVar request() {
    const char* url = "http://bce.homecomputing.fr/campaign.json";

    JSONVar json;
    WiFiClient client;

    HTTPClient http;
    http.begin(client, url);

    int code = http.GET();
    if (code < 0) {
        Serial.printf("Error[%d]: %s\n", code, http.errorToString(code).c_str());
        goto exit;
    }

    json = JSON.parse(http.getString());
    if (JSON.typeof(json) == "undefined") {
        Serial.println("Parsing input failed!");
        goto exit;
    }

exit:
    http.end();
    return json;
}

uint8_t position(JSONVar json) {
    return map((uint16_t)json["funded"], 0, (uint16_t)json["objective"], 0, NB_PIXELS - 1);
}

void gradient(uint8_t pos) {
    for (int i = 0; i < NB_PIXELS; i++) {
        if (i <= pos) {
            pixels[i] = color(i);
        } else {
            pixels[i] = CRGB::Black;
        }
    }
    FastLED.show();
}

DEFINE_GRADIENT_PALETTE(heatmap_gp) {
    0  , 0xFF, 0xFF, 0x00, // CRGB::Yellow
    80 , 0x80, 0x00, 0x80, // CRGB::Purple
    255, 0xFF, 0x00, 0x00, // CRGB::Red
};

CRGB color(uint8_t pos) {
    static CRGBPalette16 palette = heatmap_gp;
    return ColorFromPalette(palette, (uint16_t)pos * 255 / (NB_PIXELS + 2));
}
