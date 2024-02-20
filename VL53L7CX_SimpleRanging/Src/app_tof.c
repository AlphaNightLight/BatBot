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
#ifndef USE_BARE_DRIVER
static RANGING_SENSOR_Capabilities_t Cap;
#endif
static RANGING_SENSOR_ProfileConfig_t Profile;

static int32_t status = 0;
volatile uint8_t ToF_EventDetected = 0;

/* Private function prototypes -----------------------------------------------*/
#ifdef USE_BARE_DRIVER
static uint8_t map_target_status(uint8_t status);
static int32_t convert_data_format(VL53L7CX_Object_t *pObj,
                                   VL53L7CX_ResultsData *data, RANGING_SENSOR_Result_t *pResult);
#endif
static void print_result(RANGING_SENSOR_Result_t *Result);
static void toggle_resolution(void);
static void toggle_signal_and_ambient(void);
static void clear_screen(void);
static void display_commands_banner(void);
static void handle_cmd(uint8_t cmd);
static uint8_t get_key(void);
static uint32_t com_has_data(void);

void MX_TOF_Init(void)
{
  status = CUSTOM_RANGING_SENSOR_Init(CUSTOM_VL53L7CX);

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

	  status = CUSTOM_RANGING_SENSOR_Start(CUSTOM_VL53L7CX, RS_MODE_ASYNC_CONTINUOUS);

	  if (status != BSP_ERROR_NONE)
	  {
	    printf("MX_TOF_LoadDefaultConfig failed %ld\r\n", status);
	    while (1);
	  }
}

void MX_TOF_Process(void)
{
    static RANGING_SENSOR_Result_t Result;

    /* polling mode */
    status = CUSTOM_RANGING_SENSOR_GetDistance(CUSTOM_VL53L7CX, &Result);

    if (status == BSP_ERROR_NONE)
    {
    	//printf("X");
    	//fflush(stdout);
      //print_result(&Result);
    }
    else if (status != VL53L7CX_TIMEOUT)
    {
    	printf("MX_VL53L7CX_SimpleRanging_Process failed %ld\r\n", status);
    }

    if (com_has_data())
    {
      handle_cmd(get_key());
    }
}

static void print_result(RANGING_SENSOR_Result_t *Result)
{
  int8_t i;
  int8_t j;
  int8_t k;
  int8_t l;
  uint8_t zones_per_line;

  zones_per_line = ((Profile.RangingProfile == RS_PROFILE_8x8_AUTONOMOUS) ||
                    (Profile.RangingProfile == RS_PROFILE_8x8_CONTINUOUS)) ? 8 : 4;

  display_commands_banner();

  printf("Cell Format : (ranging profile = %d)\r\n\r\n", (int)Profile.RangingProfile);
  for (l = 0; l < RANGING_SENSOR_NB_TARGET_PER_ZONE; l++)
  {
    printf(" \033[38;5;10m%20s\033[0m : %20s\r\n", "Distance [mm]", "Status");
    if ((Profile.EnableAmbient != 0) || (Profile.EnableSignal != 0))
    {
      printf(" %20s : %20s\r\n", "Signal [kcps/spad]", "Ambient [kcps/spad]");
    }
  }

  printf("\r\n\r\n");

  for (j = 0; j < Result->NumberOfZones; j += zones_per_line)
  {
    for (i = 0; i < zones_per_line; i++) /* number of zones per line */
    {
      printf(" -----------------");
    }
    printf("\r\n");

    for (i = 0; i < zones_per_line; i++)
    {
      printf("|                 ");
    }
    printf("|\r\n");

    for (l = 0; l < RANGING_SENSOR_NB_TARGET_PER_ZONE; l++)
    {
      /* Print distance and status */
      for (k = (zones_per_line - 1); k >= 0; k--)
      {
        if (Result->ZoneResult[j + k].NumberOfTargets > 0)
        {
          if ((long)Result->ZoneResult[j + k].Distance[l] < 500)
          {
            printf("| \033[38;5;9m%5ld\033[0m  :  %5ld ",
                   (long)Result->ZoneResult[j + k].Distance[l],
                   (long)Result->ZoneResult[j + k].Status[l]);
          }
          else
          {
            printf("| \033[38;5;10m%5ld\033[0m  :  %5ld ",
                   (long)Result->ZoneResult[j + k].Distance[l],
                   (long)Result->ZoneResult[j + k].Status[l]);
          }
        }
        else
          printf("| %5s  :  %5s ", "X", "X");
      }
      printf("|\r\n");

      if ((Profile.EnableAmbient != 0) || (Profile.EnableSignal != 0))
      {
        /* Print Signal and Ambient */
        for (k = (zones_per_line - 1); k >= 0; k--)
        {
          if (Result->ZoneResult[j + k].NumberOfTargets > 0)
          {
            if (Profile.EnableSignal != 0)
            {
              printf("| %5ld  :  ", (long)Result->ZoneResult[j + k].Signal[l]);
            }
            else
              printf("| %5s  :  ", "X");

            if (Profile.EnableAmbient != 0)
            {
              printf("%5ld ", (long)Result->ZoneResult[j + k].Ambient[l]);
            }
            else
              printf("%5s ", "X");
          }
          else
            printf("| %5s  :  %5s ", "X", "X");
        }
        printf("|\r\n");
      }
    }
  }

  for (i = 0; i < zones_per_line; i++)
  {
    printf(" -----------------");
  }
  printf("\r\n");
}

static void toggle_resolution(void)
{
  CUSTOM_RANGING_SENSOR_Stop(CUSTOM_VL53L7CX);

  switch (Profile.RangingProfile)
  {
    case RS_PROFILE_4x4_AUTONOMOUS:
      Profile.RangingProfile = RS_PROFILE_8x8_AUTONOMOUS;
      break;

    case RS_PROFILE_4x4_CONTINUOUS:
      Profile.RangingProfile = RS_PROFILE_8x8_CONTINUOUS;
      break;

    case RS_PROFILE_8x8_AUTONOMOUS:
      Profile.RangingProfile = RS_PROFILE_4x4_AUTONOMOUS;
      break;

    case RS_PROFILE_8x8_CONTINUOUS:
      Profile.RangingProfile = RS_PROFILE_4x4_CONTINUOUS;
      break;

    default:
      break;
  }

  CUSTOM_RANGING_SENSOR_ConfigProfile(CUSTOM_VL53L7CX, &Profile);
  CUSTOM_RANGING_SENSOR_Start(CUSTOM_VL53L7CX, RS_MODE_ASYNC_CONTINUOUS);
}

static void toggle_signal_and_ambient(void)
{
  CUSTOM_RANGING_SENSOR_Stop(CUSTOM_VL53L7CX);

  Profile.EnableAmbient = (Profile.EnableAmbient) ? 0U : 1U;
  Profile.EnableSignal = (Profile.EnableSignal) ? 0U : 1U;

  CUSTOM_RANGING_SENSOR_ConfigProfile(CUSTOM_VL53L7CX, &Profile);
  CUSTOM_RANGING_SENSOR_Start(CUSTOM_VL53L7CX, RS_MODE_ASYNC_CONTINUOUS);
}

static void clear_screen(void)
{
  /* clear Screen : 27 is ESC command */
  printf("%c[2J", 27);
}

static void display_commands_banner(void)
{
  /* clear screen */
  printf("%c[2H", 27);

  printf("VL53L7CX Simple Ranging demo application\r\n");
#ifdef USE_BARE_DRIVER
  printf("Using direct calls to VL53L7CX bare driver API\r\n");
#endif
  printf("Polling mode\r\n");
  printf("----------------------------------------\r\n\r\n");

  printf("Use the following keys to control application\r\n");
  printf(" 'r' : change resolution\r\n");
  printf(" 's' : enable signal and ambient\r\n");
  printf(" 'c' : clear screen\r\n");
  printf("\r\n");
}

static void handle_cmd(uint8_t cmd)
{
  switch (cmd)
  {
    case 'r':
      toggle_resolution();
      clear_screen();
      break;

    case 's':
      toggle_signal_and_ambient();
      clear_screen();
      break;

    case 'c':
      clear_screen();
      break;

    default:
      break;
  }
}

static uint8_t get_key(void)
{
  uint8_t cmd = 0;

  HAL_UART_Receive(&hcom_uart[COM1], &cmd, 1, HAL_MAX_DELAY);

  return cmd;
}

static uint32_t com_has_data(void)
{
  return __HAL_UART_GET_FLAG(&hcom_uart[COM1], UART_FLAG_RXNE);;
}

#ifdef __cplusplus
}
#endif