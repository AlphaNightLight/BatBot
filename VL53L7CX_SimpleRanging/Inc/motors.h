#ifndef __MOTORS_H__
#define __MOTORS_H__
#include <stdint.h>

#ifdef __cplusplus
extern "C"
{
#endif

uint8_t runCar(uint8_t speed, uint8_t desired_angle, uint8_t actual_angle);

#ifdef __cplusplus
}
#endif

#endif // __MOTORS_H__
