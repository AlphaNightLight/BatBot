/**
  ******************************************************************************
  * @file          : app_tof.c
  * @author        : IMG SW Application Team
  * @brief         : This file provides code for the configuration
  *                  of the STMicroelectronics.X-CUBE-TOF1.3.4.0 instances.
  ******************************************************************************
  *
  * @attention
  *
  * Copyright (c) 2023 STMicroelectronics.
  * All rights reserved.
  *
  * This software is licensed under terms that can be found in the LICENSE file
  * in the root directory of this software component.
  * If no LICENSE file comes with this software, it is provided AS-IS.
  *
  ******************************************************************************
  */

#ifdef __cplusplus
extern "C" {
#endif

/* Includes ------------------------------------------------------------------*/
#include "app_tof.h"
#include "main.h"
#include <stdio.h>

#include "custom_ranging_sensor.h"
#include "stm32f4xx_nucleo.h"

/* Private typedef -----------------------------------------------------------*/

/* Private define ------------------------------------------------------------*/
/* uncomment following to use directly the bare driver instead of the BSP */
/* #define USE_BARE_DRIVER */
#define TIMING_BUDGET (30U) /* 5 ms < TimingBudget < 100 ms */
#define RANGING_FREQUENCY (5U) /* Ranging frequency Hz (shall be consistent with TimingBudget value) */
#define POLLING_PERIOD (1000U/RANGING_FREQUENCY) /* refresh rate for polling mode (milliseconds) */

/* Private variables ---------------------------------------------------------*/
static RANGING_SENSOR_Capabilities_t Cap;
static RANGING_SENSOR_ProfileConfig_t Profile;


void MX_TOF_Init(void)
{
	int32_t status = CUSTOM_RANGING_SENSOR_Init(CUSTOM_VL53L7CX);

  if (status != BSP_ERROR_NONE)
  {
    printf("CUSTOM_RANGING_SENSOR_Init failed %ld\r\n", status);
    while (1);
  }
}

void MX_TOF_LoadDefaultConfig(void)
{
	  uint32_t Id;

	  CUSTOM_RANGING_SENSOR_ReadID(CUSTOM_VL53L7CX, &Id);
	  CUSTOM_RANGING_SENSOR_GetCapabilities(CUSTOM_VL53L7CX, &Cap);

	  Profile.RangingProfile = RS_PROFILE_8x8_CONTINUOUS;
	  Profile.TimingBudget = TIMING_BUDGET;
	  Profile.Frequency = RANGING_FREQUENCY; /* Ranging frequency Hz (shall be consistent with TimingBudget value) */
	  Profile.EnableAmbient = 0; /* Enable: 1, Disable: 0 */
	  Profile.EnableSignal = 0; /* Enable: 1, Disable: 0 */

	  /* set the profile if different from default one */
	  CUSTOM_RANGING_SENSOR_ConfigProfile(CUSTOM_VL53L7CX, &Profile);

	  int32_t status = CUSTOM_RANGING_SENSOR_Start(CUSTOM_VL53L7CX, RS_MODE_ASYNC_CONTINUOUS);

	  if (status != BSP_ERROR_NONE)
	  {
	    printf("MX_TOF_LoadDefaultConfig failed %ld\r\n", status);
	    while (1);
	  }
}

int32_t MX_TOF_Process(RANGING_SENSOR_Result_t *pResult)
{
    int32_t status = CUSTOM_RANGING_SENSOR_GetDistance(CUSTOM_VL53L7CX, pResult);
    return status;
}

#ifdef __cplusplus
}
#endif
