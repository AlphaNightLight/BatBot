#include "integrator.h"
#include "rotation.h"
#include "data.h"
#include <stdio.h>

void integrator_calibrate(Integrator* this, AccelGyroData data) {
	this->calibratedGyro = data.gyro;
	this->calibratedAccel = data.accel;
	this->lastAccel = data.accel;

	this->velocity = VECTOR_ZERO;
	this->position = VECTOR_ZERO;
	this->rotation = VECTOR_ZERO;

	printf("Calibrating integrator with "); accel_gyro_data_print(&data); printf("\r\n");
}

int tt=0;
void integrator_update(Integrator* this, AccelGyroData data, double deltat) {
	this->rotation = vector_sum(this->rotation, vector_multiplied(vector_sum(data.gyro, vector_multiplied(this->calibratedGyro, -1)), deltat));

  Vector accel = data.accel;//rotate(data.accel, rotation);

  if (tt==0 || isnan(this->rotation.z) || isnan(this->rotation.y)){
	  printf("dt=%.5lf", deltat);
	  printf("  g_data="); vector_print(&data.gyro);
	  printf("  rotation="); vector_print(&this->rotation);
	  printf("  a_data="); vector_print(&data.accel);
	  //Vector vm = vector_multiplied(vector_sum(data.gyro, vector_multiplied(this->calibratedGyro, -1)), deltat); printf("  vm="); vector_print(&vm);
	  //printf("  a_old="); vector_print(&accel);
  }

  double lastAccelDiff = vector_length(vector_sum(accel, vector_multiplied(this->lastAccel, -1)));
  this->lastAccel = accel;
  
  accel = vector_sum(accel, vector_multiplied(this->calibratedAccel, -1));
  this->velocity = vector_sum(this->velocity, vector_multiplied(accel, deltat));
  this->position = vector_sum(this->position, vector_multiplied(this->velocity, deltat));

  if (tt==0 || isnan(this->rotation.z) || isnan(this->rotation.y)){
	  printf("  a_new="); vector_print(&accel);
	  printf("  vel="); vector_print(&this->velocity);
	  printf("  pos="); vector_print(&this->position);
	  printf("\r\n");
    tt=100;
  }
  --tt;

  if (isnan(this->rotation.z) || isnan(this->rotation.y)) while(1);
}
