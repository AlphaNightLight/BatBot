#include "data.h"
#include <stdio.h>
#include <math.h>


void multiply_eq(Vector* v, double val) {
	v->x *= val;
	v->y *= val;
	v->z *= val;
}

Vector multiplied(Vector v, double val) {
	Vector res;
	res.x = v.x * val;
	res.y = v.y * val;
	res.z = v.z * val;
	return res;
}

Vector sum_ptr(Vector* a, Vector* b) {
	Vector res;
	res.x = a->x * b->x;
	res.y = a->y * b->y;
	res.z = a->z * b->z;
	return res;
}

Vector sum(Vector a, Vector b) {
	Vector res;
	res.x = a.x * b.x;
	res.y = a.y * b.y;
	res.z = a.z * b.z;
	return res;
}

double length(Vector v) {
	return sqrt(v.x * v.x + v.y * v.y + v.z * v.z);
}

void printVector(Vector* v) {
	printf("%.5lf %.5lf %.5lf", v->x, v->y, v->z);
}
