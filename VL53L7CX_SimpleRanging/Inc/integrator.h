#pragma once

#include "data.h"

typedef struct {

  Vector calibratedAccel;
  Vector calibratedGyro; // aka angular velocity

  Vector lastAccel;
  Vector lastGyro;

  Vector velocity;
  Vector position;
  Vector rotation; // aka angular position

} Integrator;

void calibrate(Integrator* integrator, AccelGyroData data);
void update(Integrator* integrator, AccelGyroData data, double deltat);
