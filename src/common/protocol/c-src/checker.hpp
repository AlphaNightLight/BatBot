#ifndef custom_checker
#define custom_checker
#include"const.hpp"
//#include"protocol.hpp"

class Checker{
    private:
        //pointer to seiral struct
        void * data;
        int (*inner_available)(void *);
        void (*inner_send) (void*, unsigned char);
        unsigned char (*inner_read)(void *);
        void (*inner_flush)(void*);

        //internal buffers
        unsigned char buffer [BUFFER_SIZE];
        unsigned int pos=0;

        //function helpers
        void send(unsigned char);
        void flush();
        unsigned char read();
        unsigned int available(); 

        
    public:
        unsigned char out_buffer[OUT_BUFFER_SIZE];
        void send_msg(unsigned char* buff, unsigned char len);
        bool try_read_message();
        void init(void *data, int (*inner_available)(void *), void (*inner_send) (void*, unsigned char), unsigned char (*inner_read)(void *), void (*inner_flush)(void*));
};

#endif