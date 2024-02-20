#pragma once

#include "Arduino.h"
#include "triple.hpp"
#include "data.hpp"

class Integrator {

  Triple<double> calibratedAccel{};
  Triple<double> calibratedGyro{}; // aka angular velocity

  Triple<double> lastAccel{};
  Triple<double> lastGyro{};

  Triple<double> velocity{};
  Triple<double> position{};
  Triple<double> rotation{}; // aka angular position

public:

  void calibrate(const AccelGyroData& data);
  void update(const AccelGyroData& data, double deltat);

  inline const Triple<double>& getPosition() {
    return position;
  }

  inline const Triple<double>& getRotation() {
    return rotation;
  }
};