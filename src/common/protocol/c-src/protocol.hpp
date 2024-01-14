#ifndef custom_protocol
#define custom_protocol

#include"checker.hpp"

typedef enum msg_type {Ok, SpawnWall, Joystick} msg_type;

class Protocol{
    private:
        
    public:
        Checker checker;
        void init(void *data, int (*inner_available)(void *), void (*inner_send) (void*, unsigned char), unsigned char (*inner_read)(void *));
        /*unsigned char buffer [BUFFER_SIZE];
        unsigned int pos=0;
        void send(unsigned char);
    public:
        void * data;
        unsigned int (*inner_available)(void *);
        void (*inner_send) (void*, unsigned char);
        unsigned char (*inner_read)(void *);
        void send_ok( unsigned char);
        void send_msg(unsigned char* buff, unsigned char len);
        void try_read_message();
        unsigned int available(); */
};

Protocol new_protocol();

#endif