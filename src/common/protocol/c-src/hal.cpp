#include"hal.hpp"
void SerialHal::send(unsigned char t){
    this->inner_send(this->data, t);
}

unsigned int SerialHal::available(){
    return this->inner_available(this->data);
}
unsigned char SerialHal::read(){
    return this->inner_read(this->data);
}
void SerialHal::flush(){
    this->inner_flush(this->data);
}

void SerialHal::init(void *data, int (*inner_available)(void *), void (*inner_send) (void*, unsigned char), unsigned char (*inner_read)(void *), void (*inner_flush)(void*)){
    this->data=data;
    this->inner_available=inner_available;
    this->inner_send=inner_send;
    this->inner_read=inner_read;
    this->inner_flush=inner_flush;
}
SerialHal new_serial_hal(){
    SerialHal s;
    return s;
}