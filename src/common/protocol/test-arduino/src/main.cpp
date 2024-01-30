#include <Arduino.h>
#include "protocol.hpp"
#include "hal.hpp"
Protocol protocol;
SerialHal hal;

int available(void * t){
  return Serial.available();
}

void send(void * t, unsigned char c){
  Serial.write(c);
}

unsigned char read(void * t){
  return Serial.read();
}

void flush(void* t){
  Serial.flush();
}

unsigned char buffer[40];
unsigned long time;
void setup() {
  Serial.begin(115200);
  hal.init(NULL, available, send, read, flush);
  protocol.init(hal);
  for(int i=0; i<40; i++){
    buffer[i]=0;
  }
  time=millis();
}

int count =0;

void loop() {
  if(millis()-time>=14){
    time=millis();
    count ++;
    sprintf((char *) buffer , "it works %d", count);
    int len=strlen((char * )buffer);
    //buffer[len]=0;
    protocol.checker.send_msg(buffer, len);
    delay(1);
  }
  
}
