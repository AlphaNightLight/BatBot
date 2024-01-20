#include"checker.hpp"

//#include"protocol.hpp"
#include <stdio.h>
#include <stdlib.h>
/*Checker new_checker(){
    Checker t;
    return t;
}*/

void Checker::send_msg(unsigned char* buff, unsigned char len){
    this->send(START_SYMBOL);
    unsigned char chksm=MAGIC_NUMBER;
    for(unsigned char i=0; i<len; i++){
        this->send(buff[i]);
        chksm+=buff[i];
    }
    this->send(chksm);
    this->send(len);
    this->send(END_SYMBOL);
    this->flush();
}

void Checker::send(unsigned char t){
    this->inner_send(this->data, t);
}

unsigned int Checker::available(){
    return this->inner_available(this->data);
}
unsigned char Checker::read(){
    return this->inner_read(this->data);
}
void Checker::flush(){
    this->inner_flush(this->data);
}

int move(int pos){
    return (pos+BUFFER_SIZE)%BUFFER_SIZE;
}

bool Checker::try_read_message(){
    //reading all available bytes
    int max = this->available();
    for(int i=0; i<max; i++){
        //printf("%d/%d %d\n", i, max, this->pos);
        //fflush(stdout);

        int pos=this->pos;
        this->pos=move(pos+1);
        
        /*printf("pre-read %d->%d", pos, this->pos);
        fflush(stdout);*/
        if(pos>BUFFER_SIZE){
            exit(-1);
        }
        this->buffer[pos]=this->read();
        /*printf("readen %d\n", this->buffer[pos]);
        fflush(stdout);*/
        //check if a message could end here

        //it doesn't end with the correct byte
        if(this->buffer[pos]!=END_SYMBOL){
            continue;
        }
        unsigned char len = this->buffer[move(pos-1)];
        unsigned start = this->buffer[move(pos-len-3)];
        unsigned readen_chcksm=this->buffer[move(pos-2)];
        //printf("pointers: %d %d %d\n", len, start, readen_chcksm);
        //fflush(stdout);
        if(start!=START_SYMBOL){
            continue;
        }
        if(len>OUT_BUFFER_SIZE){
            continue;
        }
        pos = move(pos-len-2);
        unsigned char chcksm=MAGIC_NUMBER;
        for(int i=0; i<len; i++){
            unsigned char b= this->buffer[move(pos+i)];
            chcksm+=b;
            this->out_buffer[i]=b;
        }
        if(chcksm!=readen_chcksm){
            continue;
        }
        //we have a message TODO something
        return true;
    }
    return false;
}

void Checker::init(void *data, int (*inner_available)(void *), void (*inner_send) (void*, unsigned char), unsigned char (*inner_read)(void *), void (*inner_flush)(void*)){
    this->data=data;
    this->pos=0;
    this->inner_available=inner_available;
    this->inner_send=inner_send;
    this->inner_read=inner_read;
    this->inner_flush=inner_flush;
}