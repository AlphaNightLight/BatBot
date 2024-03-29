#include "integrator.h"
#include "rotation.h"
#include "data.h"
#include <stdio.h>

//#define INTEGRATOR_DEBUG

void integrator_calibrate(Integrator* this, AccelGyroData data) {
	this->calibratedGyro = data.gyro;
	this->calibratedAccel = data.accel;

	this->velocity = VECTOR_ZERO;
	this->position = VECTOR_ZERO;
	this->rotation = VECTOR_ZERO;

#ifdef INTEGRATOR_DEBUG
	printf("Calibrating integrator with "); accel_gyro_data_print(&data); printf("\r\n");
#endif
}

#ifdef INTEGRATOR_DEBUG
int tt=0;
#endif
void integrator_update(Integrator* this, AccelGyroData data, double deltat, CarState car_state) {
	if (car_state == INIT || car_state == STANDBY) {
		this->velocity = VECTOR_ZERO;
		return; // ignore any movement
	}

	this->rotation = vector_sum(this->rotation, vector_multiplied(vector_sum(data.gyro, vector_multiplied(this->calibratedGyro, -1)), deltat));
	this->rotation.x = 0;
	this->rotation.y = 0;

#ifdef INTEGRATOR_DEBUG
	if (tt==0){
		printf("dt=%.5lf", deltat);
		printf("  g_data="); vector_print(&data.gyro);
		printf("  rotation="); vector_print(&this->rotation);
		printf("  a_data="); vector_print(&data.accel);
		//Vector vm = vector_multiplied(vector_sum(data.gyro, vector_multiplied(this->calibratedGyro, -1)), deltat); printf("  vm="); vector_print(&vm);
		//printf("  a_old="); vector_print(&accel);
	}
#endif

	Vector accel = data.accel;
	if (car_state == FORWARD) {
		accel = rotate(&accel, &this->rotation);
		accel = vector_sum(accel, vector_multiplied(this->calibratedAccel, -1));
		accel.z = 0;
		this->velocity = vector_sum(this->velocity, vector_multiplied(accel, deltat));
		this->velocity.z = 0;
		this->position = vector_sum(this->position, vector_multiplied(this->velocity, deltat));
		this->position.z = 0;
	}

#ifdef INTEGRATOR_DEBUG
	if (tt==0) {
		printf("  a_new="); vector_print(&accel);
		printf("  vel="); vector_print(&this->velocity);
		printf("  pos="); vector_print(&this->position);
		printf("\r\n");
		tt=100;
	}
	--tt;
#endif
}
