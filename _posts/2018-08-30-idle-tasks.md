---
title: "Tasks: IdleTasks"
categories:
  - esp32
  - esp-idf
  - freertos
tags:
  - c
  - eclipse
  - "freertos tasks"

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

Running a generic IdleTask function when all your priority tasks are in the blocked state

## Notes

Check the entire code examples at the [ESP32-Repo](https://github.com/coder137/ESP32-Repo/tree/master/FreeRTOS/Task)

### Setting up your project to use `vApplicationIdleHook` for the ESP32

- `make menuconfig`
- Enter `component config`
- `FreeRTOS`
- `Use FreeRTOS legacy hooks` > Press y/Y
- `Enable legacy idle hook` > Press y/Y
- Save your changes

**NOTE: In your `main.c` file do not forget to add the function `void vApplicationIdleHook(void)`**

## Why do we need "Idle Tasks"

Most of the **priority tasks** have a fixed duration or time to run. Most of the time apart from serving these priority tasks is spent in an idle state by the microcontroller. 

However, this idle state can be utilized to get generic information about the system and run continuously when the other tasks are in a blocked state.

It is important to note that the IdleTask has a priority of 0 assigned by the FreeRTOS kernel. Hence even a Task having a priority of 1 will be given priority instead of the IdleTask.

## Using the API

**There is no API for IdleTask**

This needs to be added to your `main.c` file

``` c
void vApplicationIdleHook(void)
{
  // Write your code here
  // NOTE, Make sure it is compact
}
```

## Code Example

In this code example we will use a counter variable to store the number of cycles spent by the controller in the idle state.

``` c
static uint32_t ulIdleCycleCount;

void print_idle(void *ignore);

void app_main()
{
  xTaskCreate(&print_idle, "print task", 2048, NULL, 1, NULL);
}

// Create a periodic task that prints the idleCount value
void print_idle(void *ignore)
{
  while(1)
  {
    printf("IdleCount: %u\n", ulIdleCycleCount);
    vTaskDelay(pdMS_TO_TICKS(500));
  }
}

// Increments the idleCount variable
void vApplicationIdleHook(void)
{
  ulIdleCycleCount++;
}
```

## Output

Since my kernel is set to a speed of 100Hz this is my output

``` bash
IdleCount: 0
IdleCount: 50
IdleCount: 100
```

## Conclusion

In this tutorial we have learnt about IdleTasks, how they can be implemented in FreeRTOS and where they can be beneficial. 

In the next tutorial we shall learn about how to dynamically change the priority of different tasks. 

**Meanwhile, have fun and keep learning!**
