/**
  ******************************************************************************
  * File Name          : app_mems.c
  * Description        : This file provides code for the configuration
  *                      of the STMicroelectronics.X-CUBE-MEMS1.10.0.0 instances.
  ******************************************************************************
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
#include "app_mems.h"
#include "main.h"
#include <stdio.h>

#include "iks4a1_motion_sensors.h"
#include "stm32f4xx_nucleo.h"
#include "math.h"

/* Private variables ---------------------------------------------------------*/
static volatile uint8_t PushButtonDetected = 0;
static IKS4A1_MOTION_SENSOR_Capabilities_t MotionCapabilities[IKS4A1_MOTION_INSTANCES_NBR];

void MX_MEMS_Init(void)
{
  printf("\r\n__________________________________________________________________________\r\n");

  printf("A %ld\r\n", IKS4A1_MOTION_SENSOR_Init(IKS4A1_LSM6DSV16X_0, MOTION_ACCELERO | MOTION_GYRO));

  // printf("B %ld\r\n", IKS4A1_MOTION_SENSOR_Init(IKS4A1_LSM6DSO16IS_0, MOTION_ACCELERO | MOTION_GYRO));

  // printf("C %ld\r\n", IKS4A1_MOTION_SENSOR_Init(IKS4A1_LIS2DUXS12_0, MOTION_ACCELERO));

  //printf("D %ld\r\n", IKS4A1_MOTION_SENSOR_Init(IKS4A1_LIS2MDL_0, MOTION_MAGNETO));

  for(int i = 0; i < IKS4A1_MOTION_INSTANCES_NBR; i++)
  {
    IKS4A1_MOTION_SENSOR_GetCapabilities(i, &MotionCapabilities[i]);
    printf("\r\nMotion Sensor Instance %d capabilities: \r\n ACCELEROMETER: %d\r\n GYROSCOPE: %d\r\n MAGNETOMETER: %d\r\n LOW POWER: %d\r\n",
            i, MotionCapabilities[i].Acc, MotionCapabilities[i].Gyro, MotionCapabilities[i].Magneto, MotionCapabilities[i].LowPower);
    printf(" MAX ACC ODR: %.3f Hz, MAX ACC FS: %d\r\n",
    		MotionCapabilities[i].AccMaxOdr, (int)MotionCapabilities[i].AccMaxFS);
    printf(" MAX GYRO ODR: %.3f Hz, MAX GYRO FS: %d\r\n",
            MotionCapabilities[i].GyroMaxOdr, (int)MotionCapabilities[i].GyroMaxFS);
    printf(" MAX MAG ODR: %.3f Hz, MAX MAG FS: %d\r\n",
    		MotionCapabilities[i].MagMaxOdr, (int)MotionCapabilities[i].MagMaxFS);
  }
}

void MX_MEMS_Process(void)
{
	IKS4A1_MOTION_SENSOR_AxesRaw_t acceleration;
	IKS4A1_MOTION_SENSOR_AxesRaw_t angular_velocity;
	IKS4A1_MOTION_SENSOR_GetAxesRaw(0, MOTION_ACCELERO, &acceleration);
	IKS4A1_MOTION_SENSOR_GetAxesRaw(0, MOTION_GYRO, &angular_velocity);
}

#ifdef __cplusplus
}
#endif
