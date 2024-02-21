#ifndef __MOTORS_H__
#define __MOTORS_H__
#include <stdint.h>

typedef enum {
	INIT, STANDBY, TURNING, FORWARD
} CarState;

#ifdef __cplusplus
extern "C"
{
#endif

CarState runCar(uint8_t speed, uint8_t desired_angle, uint8_t actual_angle);

#ifdef __cplusplus
}
#endif

#endif // __MOTORS_H__
