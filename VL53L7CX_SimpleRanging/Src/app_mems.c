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
#include "integrator.h"

/* Private variables ---------------------------------------------------------*/
static IKS4A1_MOTION_SENSOR_Capabilities_t MotionCapabilities[IKS4A1_MOTION_INSTANCES_NBR];
Integrator integrator;
const double mgToMps2 = 9.80665 / 1000;
const double mdpsToRadps = 0.000017453292519943295769236907684886;
const int CALIBRATION_SKIP_COUNT = 5;
const int CALIBRATION_AVERAGE_COUNT = 10;
float acceleration_sensitivity;
float angular_velocity_sensitivity;

AccelGyroData getAccelGyroData() {
  AccelGyroData data;
  IKS4A1_MOTION_SENSOR_AxesRaw_t acceleration;
  IKS4A1_MOTION_SENSOR_AxesRaw_t angular_velocity;

  IKS4A1_MOTION_SENSOR_GetAxesRaw(0, MOTION_ACCELERO, &acceleration);
  IKS4A1_MOTION_SENSOR_GetAxesRaw(0, MOTION_GYRO, &angular_velocity);

  data.accel.x = acceleration.x * (double) acceleration_sensitivity * mgToMps2;
  data.accel.y = acceleration.y * (double) acceleration_sensitivity * mgToMps2;
  data.accel.z = acceleration.z * (double) acceleration_sensitivity * mgToMps2;
  data.gyro.x = angular_velocity.x * (double) angular_velocity_sensitivity * mdpsToRadps;
  data.gyro.y = angular_velocity.y * (double) angular_velocity_sensitivity * mdpsToRadps;
  data.gyro.z = angular_velocity.z * (double) angular_velocity_sensitivity * mdpsToRadps;

//  printf("Read ax=%d ay=%d az=%d gx=%d gy=%d gz=%d accel_gyro_data = ",
//		  acceleration.x, acceleration.y, acceleration.z,
//		  angular_velocity.x, angular_velocity.y, angular_velocity.z);
//  accel_gyro_data_print(&data); printf("\r\n");
  return data;
}

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

  IKS4A1_MOTION_SENSOR_GetSensitivity(0, MOTION_ACCELERO, &acceleration_sensitivity);
  IKS4A1_MOTION_SENSOR_GetSensitivity(0, MOTION_GYRO, &angular_velocity_sensitivity);

  AccelGyroData average = ACCEL_GYRO_DATA_ZERO;
  for (int i = 0; i < CALIBRATION_SKIP_COUNT; ++i) {
	  getAccelGyroData(); // discard data
  }
  for (int i = 0; i < CALIBRATION_AVERAGE_COUNT; ++i) {
	  average = accel_gyro_data_sum(average, getAccelGyroData());
  }
  accel_gyro_data_multiply_eq(&average, 1.0 / CALIBRATION_AVERAGE_COUNT);

  integrator_calibrate(&integrator, average);
}

void MX_MEMS_Process(CarState car_state)
{
	static uint32_t lastTime = 0;
	uint32_t curTime = DWT->CYCCNT;
	uint32_t deltaTime = curTime - lastTime;
	lastTime = curTime;
	double deltaTimeSeconds = (double) deltaTime / HAL_RCC_GetHCLKFreq();

	integrator_update(&integrator, getAccelGyroData(), deltaTimeSeconds, car_state);
}

#ifdef __cplusplus
}
#endif
