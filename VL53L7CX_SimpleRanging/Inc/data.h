#pragma once

#include <stdint.h>
 
typedef struct {
	double x, y, z;
} Vector;

typedef struct {
	Vector accel;
	Vector gyro;
} AccelGyroData;

extern const Vector ZERO_VECTOR;
extern const AccelGyroData ZERO_ACCEL_GYRO_DATA;

void vector_multiply_eq(Vector* v, double val);
Vector vector_multiplied(Vector v, double val);
Vector vector_sum_ptr(Vector* a, Vector* b);
Vector vector_sum(Vector a, Vector b);
double vector_length(Vector v);
void vector_print(Vector* v);
