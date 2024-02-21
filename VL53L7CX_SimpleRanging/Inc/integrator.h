#pragma once

#include "data.h"
#include "motors.h"

typedef struct {

  Vector calibratedAccel;
  Vector calibratedGyro; // aka angular velocity

  Vector velocity;
  Vector position;
  Vector rotation; // aka angular position

} Integrator;

void integrator_calibrate(Integrator* integrator, AccelGyroData data);
void integrator_update(Integrator* integrator, AccelGyroData data, double deltat, CarState car_state);
