#ifndef custom_checker
#define custom_checker
#include"const.hpp"
#include"hal.hpp"
//#include"protocol.hpp"
typedef struct{
    private:
        
        SerialHal serial;
        //internal buffers
        unsigned char buffer [BUFFER_SIZE];
        unsigned int pos=0;

        
        
    public:
        unsigned char out_buffer[OUT_BUFFER_SIZE];
        int out_len=0;
        void send_msg(unsigned char* buff, unsigned char len);
        bool try_read_message();
        //void init(void *data, int (*inner_available)(void *), void (*inner_send) (void*, unsigned char), unsigned char (*inner_read)(void *), void (*inner_flush)(void*));
        void init (SerialHal serial);
}Protocol;

Protocol new_protocol();

#endif