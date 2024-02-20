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
  uint32_t t0 = HAL_GetTick();
  uint32_t times = 0;
  uint32_t maxtoft = 0;
  while (1)
  {
	uint32_t toft0 = HAL_GetTick();
    MX_TOF_Process();
	uint32_t toft1 = HAL_GetTick();
	if (toft1 - toft0 > maxtoft) maxtoft = toft1 - toft0;
    MX_MEMS_Process();
    ++times;

    if (times % 1000 == 0) {
        printf("avg=%lu max=%lu last=%lu cnt=%lu\r\n", (HAL_GetTick() - t0) * 1000 / times, maxtoft, toft1-toft0, DWT->CYCCNT);
        fflush(stdout);
    }
  }
}
