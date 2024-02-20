#include "data.h"
#include <stdio.h>
#include <math.h>


const Vector VECTOR_ZERO = {0,0,0};
const AccelGyroData ACCEL_GYRO_DATA_ZERO = {VECTOR_ZERO, VECTOR_ZERO};


void vector_multiply_eq(Vector* v, double val) {
	v->x *= val;
	v->y *= val;
	v->z *= val;
}

Vector vector_multiplied(Vector v, double val) {
	Vector res;
	res.x = v.x * val;
	res.y = v.y * val;
	res.z = v.z * val;
	return res;
}

Vector vector_sum_ptr(Vector* a, Vector* b) {
	Vector res;
	res.x = a->x + b->x;
	res.y = a->y + b->y;
	res.z = a->z + b->z;
	return res;
}

Vector vector_sum(Vector a, Vector b) {
	Vector res;
	res.x = a.x + b.x;
	res.y = a.y + b.y;
	res.z = a.z + b.z;
	return res;
}

AccelGyroData accel_gyro_data_sum(AccelGyroData a, AccelGyroData b) {
	AccelGyroData res;
	res.accel = vector_sum(a.accel, b.accel);
	res.gyro = vector_sum(a.gyro, b.gyro);
	return res;
}

void accel_gyro_data_multiply_eq(AccelGyroData* v, double val) {
	vector_multiply_eq(&v->accel, val);
	vector_multiply_eq(&v->gyro, val);
}

double vector_length(Vector v) {
	return sqrt(v.x * v.x + v.y * v.y + v.z * v.z);
}

void vector_print(Vector* v) {
	printf("%+.4lf %+.4lf %+.4lf", v->x, v->y, v->z);
}

void accel_gyro_data_print(AccelGyroData* v) {
	printf("(accel = ");
	vector_print(&v->accel);
	printf("; gyro = ");
	vector_print(&v->gyro);
	printf(")");
}
