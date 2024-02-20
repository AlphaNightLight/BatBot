#pragma once

#include <stdint.h>
 
typedef struct {
	double x, y, z;
} Vector;

typedef struct {
	Vector accel;
	Vector gyro;
} AccelGyroData;

void multiply_eq(Vector* v, double val);
Vector multiplied(Vector v, double val);
Vector sum_ptr(Vector* a, Vector* b);
Vector sum(Vector a, Vector b);
double length(Vector v);
void printVector(Vector* v);
