#include "alt_main.h"
#include "protocol.hpp"
#include "main.h"
#include "app_tof.h"
#include "app_mems.h"
#include <stdio.h>
#include "hal.cpp"
#include "protocol.cpp"
unsigned char read (void * x){
	return getc(stdin);
}

void send(void *x, unsigned char c){
	putc(c, stdout);
}
int available(void *x){

	return 0;
}
void flush(void *x){
	fflush(stdout);
}

int alt_main()
{
	// Reset of all peripherals, Initializes the Flash interface and the Systick.
	  HAL_Init();
	  // initialize the DWT->CYCCNT clock cycles counter, used for precise timings
	  CoreDebug->DEMCR |= CoreDebug_DEMCR_TRCENA_Msk;
	  DWT->CYCCNT = 0;
	  DWT->CTRL |= DWT_CTRL_CYCCNTENA_Msk;

	  // Initialize all configured peripherals
	  BSP_COM_Init(COM1);
	  /*MX_TOF_Init();
	  MX_MEMS_Init();
	  MX_TOF_LoadDefaultConfig();*/

	  SerialHal serial =new_serial_hal();
	  serial.init(NULL, available, send, read, flush);
	  unsigned char s[] = "ciao\n";
	  Protocol prot = new_protocol();
	  prot.init(serial);

	  // Infinite loop
	  while (1)
	  {
		prot.send_msg(s, 5);
	    /*MX_TOF_Process();
	    MX_MEMS_Process();*/
	  }
}
