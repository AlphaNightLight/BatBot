#include "integrator.h"
#include "rotation.h"
#include "data.h"
#include <stdio.h>

void calibrate(Integrator* this, AccelGyroData data) {
	this->calibratedGyro = data.gyro;
	this->calibratedAccel = data.accel;
	this->lastAccel = data.accel;

	multiply_eq(&this->velocity, 0);
	multiply_eq(&this->position, 0);
	multiply_eq(&this->rotation, 0);
}

int tt=0;
void update(Integrator* this, AccelGyroData data, double deltat) {
	this->rotation = sum(this->rotation, multiplied(sum(data.gyro, multiplied(this->calibratedGyro, -1)), deltat));

  Vector accel = data.accel;//rotate(data.accel, rotation);

  if (tt==0){
	  printVector(&this->rotation); printf("  ");
	  printVector(&data.accel); printf("  ");
	  printVector(&accel); printf("  ");
	  printVector(&this->calibratedAccel); printf("  ");
  }

  double lastAccelDiff = length(sum(accel, multiplied(this->lastAccel, -1)));
  this->lastAccel = accel;
  
  accel = sum(accel, multiplied(this->calibratedAccel, -1));
  this->velocity = sum(this->velocity, multiplied(accel, deltat));
  this->position = sum(this->position, multiplied(this->velocity, deltat));

  if (tt==0){
	  printVector(&accel); printf("  ");
	  printVector(&this->velocity); printf("  ");
	  printVector(&this->position); printf("\r\n");
    tt=100;
  }
  --tt;
}
