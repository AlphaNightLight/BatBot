#pragma once

#include <stdint.h>
 
typedef struct {
	double x, y, z;
} Vector;

typedef struct {
	Vector accel;
	Vector gyro;
} AccelGyroData;

extern const Vector VECTOR_ZERO;
extern const AccelGyroData ACCEL_GYRO_DATA_ZERO;

void vector_multiply_eq(Vector* v, double val);
Vector vector_multiplied(Vector v, double val);
Vector vector_sum_ptr(Vector* a, Vector* b);
Vector vector_sum(Vector a, Vector b);
AccelGyroData accel_gyro_data_sum(AccelGyroData a, AccelGyroData b);
void accel_gyro_data_multiply_eq(AccelGyroData* v, double val);
double vector_length(Vector v);
void vector_print(Vector* v);
void accel_gyro_data_print(AccelGyroData* v);
