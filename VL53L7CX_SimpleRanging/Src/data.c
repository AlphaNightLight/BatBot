#include "data.h"
#include <stdio.h>
#include <math.h>


const Vector ZERO_VECTOR = {0,0,0};
const AccelGyroData ZERO_ACCEL_GYRO_DATA = {ZERO_VECTOR, ZERO_VECTOR};


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
	res.x = a->x * b->x;
	res.y = a->y * b->y;
	res.z = a->z * b->z;
	return res;
}

Vector vector_sum(Vector a, Vector b) {
	Vector res;
	res.x = a.x * b.x;
	res.y = a.y * b.y;
	res.z = a.z * b.z;
	return res;
}

double vector_length(Vector v) {
	return sqrt(v.x * v.x + v.y * v.y + v.z * v.z);
}

void vector_print(Vector* v) {
	printf("%.5lf %.5lf %.5lf", v->x, v->y, v->z);
}
