---
title: "Tasks: CreateTasks"
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
  teaser: /assets/images/freertos.jpg

# Sidebar
sidebar:
  nav: "freertos_nav"
author_profile: false
---

Get started with creating a basic task in FreeRTOS with the ESP32 and ESP-IDF

## Notes

Before starting make sure that you have downloaded the [FreeRTOS Books and Code Examples.](https://www.freertos.org/Documentation/RTOS_book.html)

Check this [tutorial]({%link _posts/2018-06-07-introduction-to-freertos.md %})

**Since we are using the ESP-IDF Framework with FreeRTOS, we do not need to call `vTaskStartScheduler()` in main. However when porting to different microcontrollers we will have to implement this.**

## What are Tasks in FreeRTOS

Each repetitive loop operation which needs to run in its own period is added into a task. If we look at back our first tutorial, [Introduction to FreeRTOS]({%link _posts/2018-06-07-introduction-to-freertos.md %}) we have taken a simple example that demonstrates this.

### Blocked State & Active State of Task

When your task is in the running state and performing its action, it is said to be in its **Active State**.

When your task is not running or waiting for an event to occur, it is said to be in its **Blocked State**.

What this means is, if there are 2 functions that both need to perform periodic tasks only one function can run at a time. This means that one function would in the running state and the other would be in the blocked state.

When the FreeRTOS Kernel *blocks* a function it lets other functions run, as opposed to blocking the entire processor using a `delay` function in Arduino.

## Implementing Tasks in FreeRTOS

The Task API is fairly complicated, however learning Tasks is integral to mastering FreeRTOS.

### Steps to Create a Task

- Create a task using the FreeRTOS `xTaskCreate` API
- Pass a specific handler/callback function that runs periodically

## CreateTasks related APIs

These are the basic _FreeRTOS Task APIs_ that we will be using in this tutorial.

### xTaskCreate

``` c
BaseType_t xTaskCreate(TaskFunction_t pvTaskCode,
                          const char * const pcName,
                          uint16_t usStackDepth,
                          void *pvParameters,
                          UBaseType_t uxPriority,
                          TaskHandle_t *pxCreatedTask)
```

*pvTaskCode*
- Add your custom function here
- These are simple C Functions that never exit and are implemented as an infinite loop

Custom function to be implemented has type
```c
void (*custom_func)(void *);
```
That is our custom function must return a void and take in a `void *` as parameter

*pcName*
- Descriptive name for the task
- Not used by FreeRTOS in any way
- Purely used as a debugging aid, human readable name

*usStackDepth*
- How much _memory_ should be allocated by the kernel to the task
- Safe amount of memory to be allocated is 2048 bytes (Specific to ESP32)
- According to FreeRTOS there is no easy way to calculate the memory requirements of a task
- Assign a reasonable value and use the FreeRTOS API to calculate the free space left over by the Task
- **Less memory causes the task to either not run or causes the ESP32 to enter kernel panic and force restart**

*pvParameters*
- Parameters to be used by the custom function are passed here
- Make sure to typecast appropriately

*uxPriority*
- The priority of the task that is running
- A higher priority number assigns a task to a higher priority. This is different from the ARM Microcontrollers where the interrupts are usually where the lower priority number is given higher priority by the microcontroller.

*pxCreatedTask*
- This is the Task Handler which is used to call the task for various operations
- The usage of this task will be covered in later tutorials

### vTaskDelay

```c
vTaskDelay(TickType_t xTicksToDelay);

//Macro to use
pdMS_TO_TICKS(uint32_t millis);
```

vTaskDelay is used to send your Task into Blocked State for a set number of Ticks

The number of Ticks, in FreeRTOS is calculated based on the TickRate. However, instead of doing the calculations on your own an easy to use Macro pdMS_TO_TICKS() converts your millisecond time into `TickType_t` that can be fed to `vTaskDelay`

## Code example

```c
void app_main()
{
    xTaskCreate(blink_task,
                "blink_task",
                configMINIMAL_STACK_SIZE,
                NULL,
                1,
                NULL);

    //Both work
    //xTaskCreate(&blink_task, "blink_task", configMINIMAL_STACK_SIZE, NULL, 1, NULL);
}

void blink_task(void *pvParameter)
{
  while(1)
  {
    printf("Led Blinking here\n");
    // ...
  }
}
```
Check the entire code [here](https://github.com/coder137/FreeRTOS_Tutorials/tree/master/Configuration/ConfigProject/import)

- In ESP-IDF instead of main, we use app_main()
- As you can see, we have set the `usStackDepth` as `configMINIMAL_STACK_SIZE` which has a default value of 768 Bytes.
- We should however change this to 2048 bytes to be on the safe side
- We have set the TaskPriority to 1, since there is no other task this is the highest priority task

**Port your code to Eclipse and build it. Check the previous tutorials on how to build your ESP-IDF project with Eclipse**

### Setting up project with Eclipse

- Clone the Code Repository [here](https://github.com/coder137/FreeRTOS_Tutorials)
- Take the ```import``` project and add the contents of the folder to your new project
- Run ```make menuconfig``` and make changes to your makefiles if needed
- Run ```make clean``` in case the project has been built before
- Import the project to Eclipse by following the previous tutorial
- Add your [Configurations](https://github.com/coder137/FreeRTOS_Tutorials/tree/master/Configuration/ConfigReadme)
- Build the project for all the packages to be detected by Eclipse

**These steps will be common in every project, follow these exact steps to get started with a new project**
{: .notice--warning}

## Output

```make monitor```

![Output](/assets/images/2018-06-25/output.png)

We learnt how to create basic Tasks and use the `xTaskCreate` API

If everything proceeds smoothly you should see your internal LED on pin D2 blinking

## Conclusion

As you can see, we successfully get our LED to blink using FreeRTOS.

However the project does not yet show the true strength of FreeRTOS Tasks. Since we are only blinking the LED, we aren't performing any multi-tasking operations.

In the next tutorial, we will learn how to add **ParameterToTasks**. We shall also create multiple Tasks and talk about something known as **Task Priority**.

**Meanwhile, have fun and keep learning!**
