#include <ti/devices/msp432p4xx/inc/msp.h>
#include <ti/devices/msp432p4xx/driverlib/driverlib.h>
#include <ti/grlib/grlib.h>
#include <stdio.h>
#include "driver.h"

#define MY_TIMER_PERIOD 255
#define TURNING_DURATION 2000 // Turning duration in milliseconds
#define DESIRED_TURN_ANGLE 90 // Desired turning angle in degrees
#define ACCEPTABLE_ERROR 5

// Timer_A Compare Configuration Parameter  (PWM) - P2.5/PM_TA0.2
Timer_A_CompareModeConfig compareConfig_PWM_DESTRA = {
TIMER_A_CAPTURECOMPARE_REGISTER_2,        // Use CCR3
        TIMER_A_CAPTURECOMPARE_INTERRUPT_DISABLE, // Disable CCR interrupt
        TIMER_A_OUTPUTMODE_TOGGLE_SET,             // Toggle output but
        0 };

// Timer_A Compare Configuration Parameter  (PWM) - P2.7/PM_TA0.4
Timer_A_CompareModeConfig compareConfig_PWM_SINISTRA = {
TIMER_A_CAPTURECOMPARE_REGISTER_4,        // Use CCR4
        TIMER_A_CAPTURECOMPARE_INTERRUPT_DISABLE, // Disable CCR interrupt
        TIMER_A_OUTPUTMODE_TOGGLE_SET,             // Toggle output but
        0 };

/* Timer_A Up Configuration Parameter */
Timer_A_UpModeConfig upConfig = {
TIMER_A_CLOCKSOURCE_SMCLK,           // SMCLK = 3 MhZ
        TIMER_A_CLOCKSOURCE_DIVIDER_12,      // SMCLK/12 = 250 KhZ
        MY_TIMER_PERIOD,                     // 20 ms tick period
        TIMER_A_TAIE_INTERRUPT_DISABLE,      // Disable Timer interrupt
        TIMER_A_CCIE_CCR0_INTERRUPT_DISABLE, // Disable CCR0 interrupt
        TIMER_A_DO_CLEAR                     // Clear value
        };

void setUp()
{
    // Configuring Timer_A0 for Up Mode and starting
    Timer_A_configureUpMode(TIMER_A0_BASE, &upConfig);
    Timer_A_startCounter(TIMER_A0_BASE, TIMER_A_UP_MODE);

    // destra
    GPIO_setAsPeripheralModuleFunctionOutputPin(
    GPIO_PORT_P2,
                                                GPIO_PIN5,
                                                GPIO_PRIMARY_MODULE_FUNCTION); // enA
    GPIO_setAsOutputPin(GPIO_PORT_P2, GPIO_PIN4); // in1
    GPIO_setAsOutputPin(GPIO_PORT_P5, GPIO_PIN6); // in2

    // sinistra
    GPIO_setAsPeripheralModuleFunctionOutputPin(
    GPIO_PORT_P2,
                                                GPIO_PIN7,
                                                GPIO_PRIMARY_MODULE_FUNCTION); // enB
    GPIO_setAsOutputPin(GPIO_PORT_P6, GPIO_PIN7); // in1
    GPIO_setAsOutputPin(GPIO_PORT_P6, GPIO_PIN6); // in2
}

void setMotorDirections(bool in1, bool in2, bool in3, bool in4)
{
    // Set IN1
    if (in1)
    {
        GPIO_setOutputHighOnPin(GPIO_PORT_P6, GPIO_PIN7);
    }
    else
    {
        GPIO_setOutputLowOnPin(GPIO_PORT_P6, GPIO_PIN7);
    }

    // Set IN2
    if (in2)
    {
        GPIO_setOutputHighOnPin(GPIO_PORT_P6, GPIO_PIN6);
    }
    else
    {
        GPIO_setOutputLowOnPin(GPIO_PORT_P6, GPIO_PIN6);
    }
    // Set IN3
    if (in3)
    {
        GPIO_setOutputHighOnPin(GPIO_PORT_P5, GPIO_PIN6);
    }
    else
    {
        GPIO_setOutputLowOnPin(GPIO_PORT_P5, GPIO_PIN6);
    }

    // Set IN4
    if (in4)
    {
        GPIO_setOutputHighOnPin(GPIO_PORT_P2, GPIO_PIN4);
    }
    else
    {
        GPIO_setOutputLowOnPin(GPIO_PORT_P2, GPIO_PIN4);
    }
}

void setMotorsForward()
{
    setMotorDirections(true, false, false, true);
}

void setMotorsTurnLeft()
{
    setMotorDirections(false, true, false, true);
}

void setMotorsTurnRight()
{
    setMotorDirections(true, false, true, false);
}

void pwmDestra(uint8_t power)
{
    compareConfig_PWM_DESTRA.compareValue = power;
    Timer_A_initCompare(TIMER_A0_BASE, &compareConfig_PWM_DESTRA);
}

void pwmSinistra(uint8_t power)
{
    compareConfig_PWM_SINISTRA.compareValue = power;
    Timer_A_initCompare(TIMER_A0_BASE, &compareConfig_PWM_SINISTRA);
}

void moveForward(uint8_t speed)
{
    setMotorsForward();
    pwmDestra(speed);
    pwmSinistra(speed);
}

void turnLeft()
{
    setMotorsTurnLeft();
    pwmDestra(127);
    pwmSinistra(127);
}

void turnRight()
{
    setMotorsTurnRight();
    pwmDestra(127);
    pwmSinistra(127);
}

void standBy()
{
    setMotorDirections(false, false, false, false);
    pwmDestra(0);
    pwmSinistra(0);
}

typedef enum
{
    INIT, STANDBY, TURNING, FORWARD
} CarState;

CarState currentState = INIT;

void transitionToState(CarState newState)
{
    // Implement any state transition actions here, if needed
    currentState = newState;
}

void updateCarState(uint8_t speed, uint8_t desired_angle, uint8_t actual_angle)
{
    switch (currentState)
    {
    case INIT:
        transitionToState(STANDBY);

        break;

    case STANDBY:
        if ((desired_angle >= actual_angle - ACCEPTABLE_ERROR)
                && (desired_angle <= actual_angle + ACCEPTABLE_ANGLE)
                && speed != 0)
        {
            transitionToState(FORWARD);

            break;
        }
        standBy();

        transitionToState(TURNING);

        break;

    case TURNING:
        if ((desired_angle >= actual_angle - ACCEPTABLE_ERROR)
                && (desired_angle <= actual_angle + ACCEPTABLE_ANGLE))
        {
            transitionToState(FORWARD);

            break;
        }

        if (desired_angle > actual_angle)
        {
            if (desired_angle - actual_angle <= 180)
            {
                turnRight();
            }
            else
            {
                turnLeft();
            }
        }
        else
        {
            if (actual_angle - desired_angle <= 180)
            {
                turnRight();
            }
            else
            {
                turnLeft();
            }
        }

        standBy();

        transitionToState(FORWARD);

        break;

    case FORWARD:
        if (speed == 0)
        {
            transitionToState(STANDBY);

            break;
        }
        if ((desired_angle <= actual_angle - ACCEPTABLE_ERROR)
                || (desired_angle >= actual_angle + ACCEPTABLE_ANGLE))
        {
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

uint8_t runCar(uint8_t speed, uint8_t desired_angle, uint8_t actual_angle)
{
    setUp();

    updateCarState();

    return currentState;
}
