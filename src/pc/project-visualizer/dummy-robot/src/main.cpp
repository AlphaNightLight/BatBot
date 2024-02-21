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

void setup() {
  Serial.begin(115200);
  hal.init(NULL, available, send, read, flush);
  protocol.init(hal);
  for(int i=0; i<10; i++){
      Serial.write(0xff);
    }
  last_cube=millis();
  last_update=millis();
  last_position_send=millis();
  pinMode(13, OUTPUT);
}

void loop() {
  if(millis()-last_cube>=30){
    last_cube=millis();
    float x = ((float)random(1000))/333.0-1.5;
    float y = ((float)random(1000))/333.0-0.5;
    float z = 0.;//((float)random(1000))/1000.0;
    memcpy(buffer, &x, sizeof(float));
    memcpy(buffer+4, &y, sizeof(float));
    memcpy(buffer+8, &z, sizeof(float));
    protocol.checker.send_msg(buffer, 12);
  }
  if(protocol.checker.try_read_message()){
    unsigned char* buff = protocol.checker.out_buffer;
    int len = protocol.checker.out_len;
    if(len==8){
      memcpy(&speed, buff, 4);
      memcpy(&rotation, buff+4, 4);
    }
    state=!state;
    if(state){
      digitalWrite(13, HIGH);
    }else{
      digitalWrite(13, LOW);
    }
  }
  if(millis() - last_position_send>=20){
    last_position_send=millis();
    memcpy(buffer, &x, sizeof(float));
    memcpy(buffer+4, &y, sizeof(float));
    memcpy(buffer+8, &z, sizeof(float));
    memcpy(buffer+12, &angle, sizeof(float));
    protocol.checker.send_msg(buffer, 16);
  } 



  unsigned long now=millis();
  float elapsed = ((float)(now-last_update))/1000;
  last_update=now;
  x+=elapsed*sin(angle)*speed*8.;
  y+=elapsed*cos(angle)*speed*8.;
  angle+=rotation*elapsed;
  delay(1);
  
}
