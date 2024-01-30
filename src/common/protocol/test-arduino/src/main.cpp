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
  for(int i=0; i<10; i++){
      Serial.write(0xff);
    }
  time=millis();
}

int count =0;

void loop() {
  if(millis()-time>=20){
    time=millis();
    count ++;
    sprintf((char *) buffer , "it works %d", count);
    int len=strlen((char * )buffer);
    //buffer[len]=0;
    /*for(int i=0; i<21-len; i++){
      Serial.write(0x11);
    }*/
    protocol.checker.send_msg(buffer, len);
    
    
    //Serial.write(0xff);
    
  }
  delay(1);
  
}
