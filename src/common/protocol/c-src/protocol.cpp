#include"protocol.hpp"

void Protocol::init(void *data, int (*inner_available)(void *), void (*inner_send) (void*, unsigned char), unsigned char (*inner_read)(void *), void (*inner_flush)(void*)){
    this->checker.init(data, inner_available, inner_send, inner_read, inner_flush);
}
Protocol new_protocol(){
    Protocol t;
    return t;
}
/*


void Protocol::send_ok(unsigned char progressive){
    unsigned char buff[2];
    buff[0]=Ok;
    buff[1]=progressive;
    this->send_msg(buff, 2);
}

void Protocol::send_msg(unsigned char* buff, unsigned char len){
    this->send(START_SYMBOL);
    unsigned char chksm=0;
    for(unsigned char i=0; i<len; i++){
        this->send(buff[i]);
        chksm+=buff[i];
    }
    this->send(chksm);
    this->send(len);
    this->send(END_SYMBOL);
}

void Protocol::send(unsigned char t){
    this->inner_send(this->data, t);
}
unsigned int Protocol::available(){
    return this->inner_available(this->data);
}

void Protocol::try_read_message(){
    
}*/