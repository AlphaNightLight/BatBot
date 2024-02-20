#include "integrator.hpp"
#include "rotation.hpp"

void Integrator::calibrate(const AccelGyroData& data) {
  calibratedGyro = data.gyro;
  calibratedAccel = data.accel;
  lastAccel = data.accel;

  velocity = 0;
  position = 0;
  rotation = 0;
}

int tt=0;
void Integrator::update(const AccelGyroData& data, double deltat) {
  rotation += (data.gyro - calibratedGyro) * deltat;

  Triple<double> accel = data.accel;//rotate(data.accel, rotation);

  if (tt==0){
    printTriple(rotation); Serial.print("  ");
    printTriple(data.accel); Serial.print(" "); Serial.print(data.accel.length()); Serial.print("  ");
    printTriple(accel); Serial.print(" "); Serial.print(accel.length()); Serial.print("  ");
    printTriple(calibratedAccel); Serial.print(" "); Serial.print(calibratedAccel.length()); Serial.print("  ");
  }

  double lastAccelDiff = (accel-lastAccel).length();
  lastAccel = accel;
  
  accel -= calibratedAccel;
  velocity += accel * deltat;
  position += velocity * deltat;

  if (tt==0){
    printTriple(accel); Serial.print(" "); Serial.print(accel.length());  Serial.print("  ");
    printTriple(velocity); Serial.print("  ");
    printTriple(position); Serial.println();
    tt=100;
  }
  --tt;
}
