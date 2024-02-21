#include "motors.h"
#include "custom_ranging_sensor.h"
#include "alt_main.h"
#include "protocol.hpp"
#include "main.h"
#include "app_tof.h"
#include "app_mems.h"
#include <stdio.h>
#include "hal.cpp"
#include "protocol.cpp"
#include <string.h>
#include <math.h>
TIM_HandleTypeDef htim3;
TIM_HandleTypeDef htim4;

UART_HandleTypeDef huart1;

static void MX_USART1_UART_Init(void);

static void MX_GPIO_Init(void);
static void MX_TIM3_Init(void);
static void MX_TIM4_Init(void);
uint8_t c = 'A';
uint8_t c2 = 'B';

#define INC_BUFFER_SIZE 100
uint8_t incoming_buf[INC_BUFFER_SIZE];
uint8_t start=0, end=0;

#define STUPID_BUFFER_SIZE 24
unsigned char STUPID_BUF[STUPID_BUFFER_SIZE];

unsigned char read (void * x){
	unsigned char val = incoming_buf[start];
	start = (start+1)%INC_BUFFER_SIZE;
	return val;
}

void send(void *x, unsigned char c){
	HAL_UART_Transmit(&huart1, &c, 1, 100);
}
int available(void *x){
	HAL_UART_Receive_IT(&huart1, incoming_buf+end, 1);
	return (end-start+INC_BUFFER_SIZE)%INC_BUFFER_SIZE;
}
void flush(void *x){
	//fflush(stdout);
}


//bool recv=false;
void HAL_UART_RxCpltCallback(UART_HandleTypeDef *huart)
{


	//recv=true;
	//HAL_UART_Receive_IT(&huart1, incoming_buf+end, 1);//You need to toggle a breakpoint on this line!
	HAL_UART_Receive_IT(&huart1, STUPID_BUF, 10);
	for(int i=0; i<10; i++){
		incoming_buf[end]=STUPID_BUF[i];
		end = (end+1)%INC_BUFFER_SIZE;
		if(end==0){
			int t =0;
		}
	}


}


int random(int x){
	return 0;
}
int millis(){
	return (uint32_t)((uint64_t)DWT->CYCCNT*1000/HAL_RCC_GetHCLKFreq());
}

void test_protocol (){ //TODO LEVAMI
	Protocol protocol;
	SerialHal hal;
	hal.init(NULL, available, send, read, flush);
	protocol.init(hal);

	unsigned char buffer[40];
	unsigned long last_cube=millis();
	unsigned long last_position_send=millis();

	float x=0.0, y=0.0, z=0.0, angle=0.0;
	float speed=0.0, rotation=1.0;
	unsigned long last_update;

	while(1){
		if(millis()-last_cube>=30){
		    last_cube=millis();
		    float x = ((float)random(1000))/333.0-1.5;
		    float y = ((float)random(1000))/333.0-0.5;
		    float z = 0.;//((float)random(1000))/1000.0;
		    memcpy(buffer, &x, sizeof(float));
		    memcpy(buffer+4, &y, sizeof(float));
		    memcpy(buffer+8, &z, sizeof(float));
		    protocol.send_msg(buffer, 12);
		  }
		  if(protocol.try_read_message()){
		    unsigned char* buff = protocol.out_buffer;
		    int len = protocol.out_len;
		    if(len==8){
		      memcpy(&speed, buff, 4);
		      memcpy(&rotation, buff+4, 4);
		    }
		  }
		  if(millis() - last_position_send>=20){
		    last_position_send=millis();
		    memcpy(buffer, &x, sizeof(float));
		    memcpy(buffer+4, &y, sizeof(float));
		    memcpy(buffer+8, &z, sizeof(float));
		    memcpy(buffer+12, &angle, sizeof(float));
		    protocol.send_msg(buffer, 16);
		  }



		  unsigned long now=millis();
		  float elapsed = 0.001;//((float)(now-last_update))/1000;
		  last_update=now;
		  x+=elapsed*sin(angle)*speed*8.;
		  y+=elapsed*cos(angle)*speed*8.;
		  angle+=rotation*elapsed;
		  HAL_Delay(1);
	}
}

int alt_main()
{
	// Reset of all peripherals, Initializes the Flash interface and the Systick.
	  HAL_Init();
	  // initialize the DWT->CYCCNT clock cycles counter, used for precise timings
	  CoreDebug->DEMCR |= CoreDebug_DEMCR_TRCENA_Msk;
	  DWT->CYCCNT = 0;
	  DWT->CTRL |= DWT_CTRL_CYCCNTENA_Msk;

  // initialize timers for PWM
  MX_GPIO_Init();

  MX_USART1_UART_Init();
  HAL_UART_Receive_IT(&huart1, &c, 1);

  /*while(1){
	  HAL_UART_Receive_IT(&huart1, &c, 1);
	  HAL_Delay(100);
	  HAL_UART_Transmit(&huart1, &c, 1, 100);
  }*/
  test_protocol();

  MX_TIM3_Init();
  MX_TIM4_Init();
  TIM3->CCR1 = 0;
  HAL_TIM_PWM_Start(&htim3, TIM_CHANNEL_1);
  TIM4->CCR1 = 0;
  HAL_TIM_PWM_Start(&htim4, TIM_CHANNEL_1);
//  uint8_t x=0;
//  bool dir = false;
//  while (1)
//  {
//	  x += 1;
//	  TIM3->CCR1 = x;
//	  TIM4->CCR1 = x;
//	  HAL_Delay(10);
//
//	  if (x==0){
//		  dir = !dir;
//	  }
//
//	  HAL_GPIO_WritePin(GPIOB, GPIO_PIN_10, dir ? GPIO_PinState::GPIO_PIN_RESET : GPIO_PinState::GPIO_PIN_SET);
//	  HAL_GPIO_WritePin(GPIOC, GPIO_PIN_7, dir ? GPIO_PinState::GPIO_PIN_RESET : GPIO_PinState::GPIO_PIN_SET);
//	  HAL_GPIO_WritePin(GPIOA, GPIO_PIN_8, dir ? GPIO_PinState::GPIO_PIN_SET : GPIO_PinState::GPIO_PIN_RESET);
//	  HAL_GPIO_WritePin(GPIOA, GPIO_PIN_9, dir ? GPIO_PinState::GPIO_PIN_SET : GPIO_PinState::GPIO_PIN_RESET);
//  }

	  // Initialize all configured peripherals
	  //BSP_COM_Init(COM1);
	  /*MX_TOF_Init();
	  MX_MEMS_Init();
	  MX_TOF_LoadDefaultConfig();*/

	  SerialHal serial =new_serial_hal();
	  serial.init(NULL, available, send, read, flush);
	  unsigned char s[] = "ciao\n";
	  Protocol prot = new_protocol();
	  prot.init(serial);

	  // Infinite loop
	  while (1)
	  {
		prot.send_msg(s, 5);
	    /*MX_TOF_Process();
	    MX_MEMS_Process();*/
	  }

	  RANGING_SENSOR_Result_t TOF_data;
	  while (1)
	  {
		  int32_t TOF_status = MX_TOF_Process(&TOF_data);
		  if (TOF_status == BSP_ERROR_NONE) {
			  // TODO INVIARLO CON PROTOCOLLO
		  }

		  // TODO PRENDERE DATI DA PROTOCOLLO
		  CarState returnedState = runCar(250, 0, integrator.rotation.z);
		  MX_MEMS_Process(returnedState);
	  }
}





/**
  * @brief USART2 Initialization Function
  * @param None
  * @retval None
  */
static void MX_USART1_UART_Init(void)
{

  /* USER CODE BEGIN USART2_Init 0 */

  /* USER CODE END USART2_Init 0 */

  /* USER CODE BEGIN USART2_Init 1 */

  /* USER CODE END USART2_Init 1 */
  huart1.Instance = USART1;
  huart1.Init.BaudRate = 115200;
  huart1.Init.WordLength = UART_WORDLENGTH_8B;
  huart1.Init.StopBits = UART_STOPBITS_1;
  huart1.Init.Parity = UART_PARITY_NONE;
  huart1.Init.Mode = UART_MODE_TX_RX;
  huart1.Init.HwFlowCtl = UART_HWCONTROL_NONE;
  huart1.Init.OverSampling = UART_OVERSAMPLING_16;
  if (HAL_UART_Init(&huart1) != HAL_OK)
  {
    Error_Handler();
  }
  /* USER CODE BEGIN USART2_Init 2 */

  /* USER CODE END USART2_Init 2 */

}


/**
  * @brief TIM3 Initialization Function
  * @param None
  * @retval None
  */
static void MX_TIM3_Init(void)
{

  /* USER CODE BEGIN TIM3_Init 0 */

  /* USER CODE END TIM3_Init 0 */

  TIM_ClockConfigTypeDef sClockSourceConfig = {0};
  TIM_MasterConfigTypeDef sMasterConfig = {0};
  TIM_OC_InitTypeDef sConfigOC = {0};

  /* USER CODE BEGIN TIM3_Init 1 */

  /* USER CODE END TIM3_Init 1 */
  htim3.Instance = TIM3;
  htim3.Init.Prescaler = 16-1;
  htim3.Init.CounterMode = TIM_COUNTERMODE_UP;
  htim3.Init.Period = 255-1;
  htim3.Init.ClockDivision = TIM_CLOCKDIVISION_DIV1;
  htim3.Init.AutoReloadPreload = TIM_AUTORELOAD_PRELOAD_DISABLE;
  if (HAL_TIM_Base_Init(&htim3) != HAL_OK)
  {
    Error_Handler();
  }
  sClockSourceConfig.ClockSource = TIM_CLOCKSOURCE_INTERNAL;
  if (HAL_TIM_ConfigClockSource(&htim3, &sClockSourceConfig) != HAL_OK)
  {
    Error_Handler();
  }
  if (HAL_TIM_PWM_Init(&htim3) != HAL_OK)
  {
    Error_Handler();
  }
  sMasterConfig.MasterOutputTrigger = TIM_TRGO_RESET;
  sMasterConfig.MasterSlaveMode = TIM_MASTERSLAVEMODE_DISABLE;
  if (HAL_TIMEx_MasterConfigSynchronization(&htim3, &sMasterConfig) != HAL_OK)
  {
    Error_Handler();
  }
  sConfigOC.OCMode = TIM_OCMODE_PWM1;
  sConfigOC.Pulse = 0;
  sConfigOC.OCPolarity = TIM_OCPOLARITY_HIGH;
  sConfigOC.OCFastMode = TIM_OCFAST_DISABLE;
  if (HAL_TIM_PWM_ConfigChannel(&htim3, &sConfigOC, TIM_CHANNEL_1) != HAL_OK)
  {
    Error_Handler();
  }
  /* USER CODE BEGIN TIM3_Init 2 */

  /* USER CODE END TIM3_Init 2 */
  HAL_TIM_MspPostInit(&htim3);

}

/**
  * @brief TIM4 Initialization Function
  * @param None
  * @retval None
  */
static void MX_TIM4_Init(void)
{

  /* USER CODE BEGIN TIM4_Init 0 */

  /* USER CODE END TIM4_Init 0 */

  TIM_ClockConfigTypeDef sClockSourceConfig = {0};
  TIM_MasterConfigTypeDef sMasterConfig = {0};
  TIM_OC_InitTypeDef sConfigOC = {0};

  /* USER CODE BEGIN TIM4_Init 1 */

  /* USER CODE END TIM4_Init 1 */
  htim4.Instance = TIM4;
  htim4.Init.Prescaler = 16-1;
  htim4.Init.CounterMode = TIM_COUNTERMODE_UP;
  htim4.Init.Period = 255-1;
  htim4.Init.ClockDivision = TIM_CLOCKDIVISION_DIV1;
  htim4.Init.AutoReloadPreload = TIM_AUTORELOAD_PRELOAD_DISABLE;
  if (HAL_TIM_Base_Init(&htim4) != HAL_OK)
  {
    Error_Handler();
  }
  sClockSourceConfig.ClockSource = TIM_CLOCKSOURCE_INTERNAL;
  if (HAL_TIM_ConfigClockSource(&htim4, &sClockSourceConfig) != HAL_OK)
  {
    Error_Handler();
  }
  if (HAL_TIM_PWM_Init(&htim4) != HAL_OK)
  {
    Error_Handler();
  }
  sMasterConfig.MasterOutputTrigger = TIM_TRGO_RESET;
  sMasterConfig.MasterSlaveMode = TIM_MASTERSLAVEMODE_DISABLE;
  if (HAL_TIMEx_MasterConfigSynchronization(&htim4, &sMasterConfig) != HAL_OK)
  {
    Error_Handler();
  }
  sConfigOC.OCMode = TIM_OCMODE_PWM1;
  sConfigOC.Pulse = 0;
  sConfigOC.OCPolarity = TIM_OCPOLARITY_HIGH;
  sConfigOC.OCFastMode = TIM_OCFAST_DISABLE;
  if (HAL_TIM_PWM_ConfigChannel(&htim4, &sConfigOC, TIM_CHANNEL_1) != HAL_OK)
  {
    Error_Handler();
  }
  /* USER CODE BEGIN TIM4_Init 2 */

  /* USER CODE END TIM4_Init 2 */
  HAL_TIM_MspPostInit(&htim4);

}

/**
  * @brief GPIO Initialization Function
  * @param None
  * @retval None
  */
static void MX_GPIO_Init(void)
{
	  GPIO_InitTypeDef GPIO_InitStruct = {0};
/* USER CODE BEGIN MX_GPIO_Init_1 */
/* USER CODE END MX_GPIO_Init_1 */

  /* GPIO Ports Clock Enable */
  __HAL_RCC_GPIOB_CLK_ENABLE();
  __HAL_RCC_GPIOC_CLK_ENABLE();
  __HAL_RCC_GPIOA_CLK_ENABLE();

  /*Configure GPIO pin Output Level */
  HAL_GPIO_WritePin(GPIOB, GPIO_PIN_10, GPIO_PIN_RESET);

  /*Configure GPIO pin Output Level */
  HAL_GPIO_WritePin(GPIOC, GPIO_PIN_7, GPIO_PIN_RESET);

  /*Configure GPIO pin Output Level */
  HAL_GPIO_WritePin(GPIOA, GPIO_PIN_8|GPIO_PIN_9, GPIO_PIN_RESET);

  /*Configure GPIO pin : PB10 */
  GPIO_InitStruct.Pin = GPIO_PIN_10;
  GPIO_InitStruct.Mode = GPIO_MODE_OUTPUT_PP;
  GPIO_InitStruct.Pull = GPIO_NOPULL;
  GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_LOW;
  HAL_GPIO_Init(GPIOB, &GPIO_InitStruct);

  /*Configure GPIO pin : PC7 */
  GPIO_InitStruct.Pin = GPIO_PIN_7;
  GPIO_InitStruct.Mode = GPIO_MODE_OUTPUT_PP;
  GPIO_InitStruct.Pull = GPIO_NOPULL;
  GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_LOW;
  HAL_GPIO_Init(GPIOC, &GPIO_InitStruct);

  /*Configure GPIO pins : PA8 PA9 */
  GPIO_InitStruct.Pin = GPIO_PIN_8|GPIO_PIN_9;
  GPIO_InitStruct.Mode = GPIO_MODE_OUTPUT_PP;
  GPIO_InitStruct.Pull = GPIO_NOPULL;
  GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_LOW;
  HAL_GPIO_Init(GPIOA, &GPIO_InitStruct);

/* USER CODE BEGIN MX_GPIO_Init_2 */
/* USER CODE END MX_GPIO_Init_2 */
}


void send_to_display(Protocol &protocol, RANGING_SENSOR_Result_t to_send){
	uint8_t tmp[33];
	for (uint8_t i=0; i<8; i++){
		tmp[0]=i;
		for(unint8_t x=0; x<8; x++){
			memcpy(tmp+1+x*4, to_send.ZoneResult+i*8+x, sizeof(float));
		}
		protocol.send_msg(tmp, 33);
	}

}
/**
  * @brief  This function is executed in case of error occurrence.
  * @retval None
  */
void Error_Handler(void)
{
  /* USER CODE BEGIN Error_Handler_Debug */
  /* User can add his own implementation to report the HAL error return state */
  __disable_irq();
  while (1)
  {
  }
  /* USER CODE END Error_Handler_Debug */
}
