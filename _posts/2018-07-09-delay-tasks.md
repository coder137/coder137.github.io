---
title: "Tasks: DelayTasks"
categories:
  - esp32
  - esp-idf
  - freertos
tags:
  - c
  - eclipse
  - "freertos tasks"

date: 2018-07-30

# Added table of contents defn
toc: true
toc_label: "Table of Contents"
toc_icon: "cog"

# header
header:
  teaser: /assets/images/teaser_images/tasks.jpg

# Sidebar
sidebar:
  nav: "freertos_nav"
author_profile: false
---

Setting a software delay in your FreeRTOS tasks, that enables other waiting tasks to run.

## Notes

Check the [first Task tutorial]({% link _posts/2018-06-25-create-tasks.md %}) where the vTaskDelay API was discussed.

**Never use Software delays such as these in any Hardware or Software Interrupt. This is bad practice and can cause your ESP32 to enter Kernel panic and force restart**

## Why do we need to "Delay Tasks"

Looking back at our first tutorial [Introduction to FreeRTOS]({% link _posts/2018-06-07-introduction-to-freertos.md %}), this following example was given

"Say you need to read an Analog Input from a device and subsequently drive a motor. However you also want to send an SMS using a GSM Module every 20 seconds"

As you can see, in the above example there are 2 tasks taking place

- Analog Input and driving a motor
  - Happening as frequently as possible, i.e a 10ms delay at max (stability)
- Sending an SMS using a GSM Module 
  - Every 20 seconds

As you can see, every task has their own delay requirements

1. Time Critical Delays (SMS Example)
2. Stability related Delays (Analog Motor Example)
3. Conditional delays, for example Pulse Skip Modulation for AC Current

are some examples of where delays could be very beneficial.

### Why not use Hardware Timers for delays

Although using Hardware Timers for delays is much more efficient than software based delays, ultimately the availability of such Timers depends on your Hardware. 

Moreover in a complex or large implementation you might not have the luxury of using Hardware Timers
(either due to cost, space or internal hardware constraints) and hence would have to resort to using Software based delays.

## Using the API

### pdMS_TO_TICKS

``` c
TickType_t pdMS_TO_TICKS(uint32_t millis);
```

The `pdMS_TO_TICKS()` API converts your millisecond requirement to FreeRTOS Ticks. 

The FreeRTOS Kernel uses Ticks to schedule and keep track of the various tasks in a microcontroller up to 1KHZ.

**Example**

If your FreeRTOS is configured to use a frequency of 100HZ, your tick rate would be 10ms. If the task needs to be delayed by 500ms, the `TickType_t` would be 50 (500/10).

To avoid unnecessary math and computation on the user's end the MACRO `pdMS_TO_TICKS` is very useful.

``` c
// If FreeRTOS Freq = 100 (HZ)
TickType_t xDelay500ms = pdMS_TO_TICKS(500);
//TickType_t xDelay500ms has a value of 50
```

### vTaskDelay

``` c
void vTaskDelay(TickType_t xTicksToDelay);
```

This function sends that particular task into the blocked state for a set amount of Ticks.

### vTaskDelayUntil

```c
void vTaskDelayUntil(TickType_t *pxPreviousWakeTime, TickType_t xTimeIncrement);
```

Places the task that calls this function into the Blocked state until that absolute time is reached.

This function is very useful for Periodic Tasks where a constant execution frequency is of the utmost importance

## Code Example (vTaskDelay)

``` c
const TickType_t xDelay250ms = pdMS_TO_TICKS(250);

// app_main runs here

void print_task(void *pvParameters)
{
  char *pcTaskName;
  pcTaskName = (char *)pvParameters;

  while(1)
  {
    printf("Parameter: %s", pcTaskName);
    vTaskDelay(xDelay250ms);
  }
  vTaskDelete(NULL);
}
```

Entire code is present [here](https://github.com/coder137/ESP32-Repo/blob/master/FreeRTOS/Task/DelayTasks/main/main.c)


## Code Example (vTaskDelayUntil)

``` c
void vTaskFunction( void *pvParameters )
{
  char *pcTaskName;
  TickType_t xLastWakeTime;

  pcTaskName = ( char * ) pvParameters;

  // We do this only once
  xLastWakeTime = xTaskGetTickCount();

  for( ;; )
  {
    printf("Task: %s\n", pcTaskName);

    // variable xLastWakeTime is updated internally
    vTaskDelayUntil(&xLastWakeTime, pdMS_TO_TICKS( 250 ));
  }
}
```

Entire code for `vTaskDelayUntil` present [here](https://github.com/coder137/ESP32-Repo/blob/master/FreeRTOS/Task/DelayTasksUntil/main/main.c)

## Output (vTaskDelay AND vTaskDelayUntil)

As per the code present in the code repository

- The Internal LED at GPIO2 will be blinking at 250ms period
- Output on Serial Monitor `print_task`

``` bash
Parameter: Task 1
Parameter: Task 2
Parameter: Task 1
Parameter: Task 2
```
In our example above we are using 3 multitasking functions

## Difference between vTaskDelay and vTaskDelayUntil

Since the output for vTaskDelay and vTaskDelayUntil is same, we should note the key differences between the two.

<!-- Do not touch the below table code (https://www.tablesgenerator.com/markdown_tables) -->

| vTaskDelay 	| vTaskDelayUntil 	|
|:----------------------------------------------------------------------------:	|:---------------------------------------------------------------------------------------------------------------------------------------------------------:	|
| In vTaskDelay you say how long after calling vTaskDelay you want to be woken 	| In vTaskDelayUntil you say the **time** at which you want to be woken 	|
| The parameter in vTaskDelay is the delay period in number of ticks from now 	| The parameter in vTaskDelayUntil is the **absolute time** in ticks at which you want to be woken calculated as an increment from the time you were last woken 	|
| vTaskDelay is **relative** to the function itself 	| vTaskDelayUntil is **absolute** in terms of the ticks set by scheduler and FreeRTOS Kernel 	|

## Conclusion

In this tutorial we have seen why delaying a Task is beneficial and how it can be used in various scenarios and implemented in FreeRTOS.

In the next tutorial we shall talk about Idle Tasks which have the lowest priority in FreeRTOS and which runs when all the other functions having a higher priority are in the blocked state.

**Meanwhile, have fun and keep learning!**