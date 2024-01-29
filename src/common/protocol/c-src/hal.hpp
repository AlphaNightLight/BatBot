#ifndef custom_protocol_hal
#define custom_protocol_hal

class SerialHal{
    private:
        void * data;
        int (*inner_available)(void *);
        void (*inner_send) (void*, unsigned char);
        unsigned char (*inner_read)(void *);
        void (*inner_flush)(void*);
    public:
        void init();
        void flush();
        void send(unsigned char);
        unsigned char read();
        unsigned int available();
        void init(void *data, int (*inner_available)(void *), void (*inner_send) (void*, unsigned char), unsigned char (*inner_read)(void *), void (*inner_flush)(void*));
};
SerialHal new_serial_hal();
#endif