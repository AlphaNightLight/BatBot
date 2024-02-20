#include "data.hpp"

AccelGyroData AccelGyroData::operator+=(const AccelGyroData& o) {
  return *this = *this + o;
}

AccelGyroData AccelGyroData::operator-=(const AccelGyroData& o) {
  return *this = *this - o;
}

AccelGyroData AccelGyroData::operator/=(double o) {
  return *this = *this / o;
}

AccelGyroData AccelGyroData::operator*=(double o) {
  return *this = *this * o;
}
