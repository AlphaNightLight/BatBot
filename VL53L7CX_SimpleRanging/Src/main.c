#include "main.h"
#include "app_tof.h"
#include "app_mems.h"
#include <stdio.h>

int main(void)
{
  // Reset of all peripherals, Initializes the Flash interface and the Systick.
  HAL_Init();

  // initialize the DWT->CYCCNT clock cycles counter, used for precise timings
  CoreDebug->DEMCR |= CoreDebug_DEMCR_TRCENA_Msk;
  DWT->CYCCNT = 0;
  DWT->CTRL |= DWT_CTRL_CYCCNTENA_Msk;

  // Initialize all configured peripherals
  BSP_COM_Init(COM1);
  MX_TOF_Init();
  MX_MEMS_Init();
  MX_TOF_LoadDefaultConfig();

  // Infinite loop
  while (1)
  {
    MX_TOF_Process();
    MX_MEMS_Process();
  }
}
