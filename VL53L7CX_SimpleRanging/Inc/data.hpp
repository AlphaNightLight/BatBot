#pragma once

#include "Arduino.h"
#include "triple.hpp"

struct AccelGyroData {
  Triple<double> accel;
  Triple<double> gyro;

  constexpr AccelGyroData() : accel{}, gyro{} {};
  constexpr AccelGyroData(Triple<double> accel_, Triple<double> gyro_) : accel{accel_}, gyro{gyro_} {};

  constexpr AccelGyroData operator+() const {
    return *this;
  }

  constexpr AccelGyroData operator-() const {
    return AccelGyroData{-accel, -gyro};
  }

  constexpr AccelGyroData operator+(const AccelGyroData& o) const {
    return AccelGyroData{accel + o.accel, gyro + o.gyro};
  }

  constexpr AccelGyroData operator-(const AccelGyroData& o) const {
    return operator+(-o);
  }

  constexpr AccelGyroData operator/(double o) const {
    return AccelGyroData{accel / o, gyro / o};
  }

  constexpr AccelGyroData operator*(double o) const {
    return AccelGyroData{accel * o, gyro * o};
  }

  AccelGyroData operator+=(const AccelGyroData& o);
  AccelGyroData operator-=(const AccelGyroData& o);
  AccelGyroData operator/=(double o);
  AccelGyroData operator*=(double o);
}; 
 
