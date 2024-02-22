#include <stdio.h>
#include <stdbool.h>
#include "main.h"
#include "motors.h"

#define ACCEPTABLE_ERROR 5
#define TURNING_SPEED 200

void setMotorDirections(bool in1, bool in2, bool in3, bool in4) {
	// Set IN1
	if (in1) {
		HAL_GPIO_WritePin(GPIOB, GPIO_PIN_10, GPIO_PIN_SET);
	} else {
		HAL_GPIO_WritePin(GPIOB, GPIO_PIN_10, GPIO_PIN_RESET);
	}

	// Set IN2
	if (in2) {
		HAL_GPIO_WritePin(GPIOA, GPIO_PIN_8, GPIO_PIN_SET);
	} else {
		HAL_GPIO_WritePin(GPIOA, GPIO_PIN_8, GPIO_PIN_RESET);
	}
	// Set IN3
	if (in3) {
		HAL_GPIO_WritePin(GPIOB, GPIO_PIN_3, GPIO_PIN_SET);
	} else {
		HAL_GPIO_WritePin(GPIOB, GPIO_PIN_3, GPIO_PIN_RESET);
	}

	// Set IN4
	if (in4) {
		HAL_GPIO_WritePin(GPIOC, GPIO_PIN_7, GPIO_PIN_SET);
	} else {
		HAL_GPIO_WritePin(GPIOC, GPIO_PIN_7, GPIO_PIN_RESET);
	}
}

void setMotorsForward() {
	setMotorDirections(true, false, false, true);
}

void setMotorsTurnLeft() {
	setMotorDirections(false, true, false, true);
}

void setMotorsTurnRight() {
	setMotorDirections(true, false, true, false);
}

void pwmDestra(uint8_t power) {
	TIM4->CCR1 = power;
}

void pwmSinistra(uint8_t power) {
	TIM3->CCR1 = power;
}

void moveForward(uint8_t speed) {
	setMotorsForward();
	pwmDestra(speed);
	pwmSinistra(speed);
}

void turnLeft() {
	setMotorsTurnLeft();
	pwmDestra(TURNING_SPEED);
	pwmSinistra(TURNING_SPEED);
}

void turnRight() {
	setMotorsTurnRight();
	pwmDestra(TURNING_SPEED);
	pwmSinistra(TURNING_SPEED);
}

void standBy() {
	setMotorDirections(false, false, false, false);
	pwmDestra(0);
	pwmSinistra(0);
}

CarState currentState = INIT;

void transitionToState(CarState newState) {
	// Implement any state transition actions here, if needed
	currentState = newState;
}

void updateCarState(uint8_t speed, uint8_t desired_angle, uint8_t actual_angle) {
	switch (currentState) {
	case INIT:
		transitionToState(STANDBY);

		break;

	case STANDBY:
		if ((desired_angle >= actual_angle - ACCEPTABLE_ERROR)
				&& (desired_angle <= actual_angle + ACCEPTABLE_ERROR)
				&& speed != 0) {
			transitionToState(FORWARD);
			break;
		} else if (speed != 0) {
			transitionToState(TURNING);
			break;
		}
		standBy();

		break;

	case TURNING:
		if (speed == 0) {
			transitionToState(STANDBY);
			break;
		}
		if ((desired_angle >= actual_angle - ACCEPTABLE_ERROR)
				&& (desired_angle <= actual_angle + ACCEPTABLE_ERROR)) {
			transitionToState(FORWARD);

			break;
		}

		if (desired_angle > actual_angle) {
			if (desired_angle - actual_angle <= 180) {
				turnRight();
			} else {
				turnLeft();
			}
		} else {
			if (actual_angle - desired_angle <= 180) {
				turnRight();
			} else {
				turnLeft();
			}
		}

		break;

	case FORWARD:
		if (speed == 0) {
			transitionToState(STANDBY);

			break;
		}
		if ((desired_angle <= actual_angle - ACCEPTABLE_ERROR)
				|| (desired_angle >= actual_angle + ACCEPTABLE_ERROR)) {
			transitionToState(TURNING);

			break;
		}
		moveForward(speed);

		transitionToState(STANDBY);

		break;

	default:
		printf("Error in the FSM switch\n");

		break;
	}
}

CarState runCar(uint8_t speed, double desired_angle, double actual_angle) {
	updateCarState(speed, desired_angle, actual_angle);

	return currentState;
}
