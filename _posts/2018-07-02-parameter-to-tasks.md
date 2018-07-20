---
title: "Tasks: ParameterToTasks"
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

Learn to add dynamic parameter to Tasks in FreeRTOS with the ESP32 and ESP-IDF

## Notes

Check the previous tutorial on [Creating a Basic Task]({%link _posts/2018-06-25-create-tasks.md%})

Check this tutorial on [Pointer to void]({%link _posts/2018-06-08-pointer-to-void.md %})

Import your project from one of [these configured builds](https://github.com/coder137/ESP32-Repo/tree/master/Configs/Configured)

## Why do we need to add "Parameters to Tasks"

In many cases, we would need to change the usage of a function with the Parameter inputs to a function. 

It would be highly inefficient to copy an existing function and make just a slight modification to it.

## Using the `xTaskCreate` API

``` c
BaseType_t xTaskCreate(TaskFunction_t pvTaskCode,
                          const char * const pcName,
                          uint16_t usStackDepth,
                          void *pvParameters,
                          UBaseType_t uxPriority,
                          TaskHandle_t *pxCreatedTask)
```

We learnt the usage of this API is the [previous tutorial]({%link _posts/2018-06-25-create-tasks.md%})

However, since we just needed to create Basic Tasks we did not use many parameters, mainly the `void *pvParameters` and the `TaskHandle_t *pxCreatedTask` parameters.

In this tutorial we will learn how to use the `void *pvParameters` parameter pertaining to the `xTaskCreate() API`

## Code Example

``` c 
const char *pcTask1 = "Task1\n"
const char *pcTask2 = "Task2\n"

void vPrintFunction(void *parameter);

void app_main()
{
  xTaskCreate(vPrintFunction, "Print100", 2048, (void *) pcTask1, 1, NULL);
  xTaskCreate(vPrintFunction, "Print200", 2048, (void *) pcTask2, 1, NULL);
}

void vPrintFunction(void *parameter)
{
  char *pcTaskName;
  pcTaskName = (char *) parameter;

  while(1)
  {
    printf("Parameter: %s", pcTaskName);
    vTaskDelay(1000/portTICK_PERIOD_MS);
  }
}
```

Check the entire code [here](https://github.com/coder137/ESP32-Repo/tree/master/FreeRTOS/Task/ParameterToTasks)

## Output

```
Parameter: Task 1
Parameter: Task 2
Parameter: Task 2
Parameter: Task 1
Parameter: Task 1
Parameter: Task 2
```

The output on your terminal would be something akin to this.
As you can see Task2 runs after Task1 after the first iteration.

### Why does this happen?

This is because both Task1 and Task2 have the same priority to run. When the program finishes one iteration, during the second function iteration both functions have equal priority to run. 

This might cause either Task1 or Task2 to be chosen, i.e both functions have an equal probability to be executed.

*We shall talk about task priorities and how they function more in later tutorials too*

### Sending a struct as a parameter

While the above example does use only `const char *` supplied as a `void *parameter` we can very well supply a `struct` to our program.

``` c
typedef struct Data_t
{
    uint32_t ucData;
    char id;
} GenericData_t;

void genericStructPrint(void *xStruct)
{
    GenericData_t * data = (GenericData_t *) xStruct;
    printf("ucData: %" PRIu32 "\n", data->ucData);
    printf("id: %c\n", data->id);
}

int main()
{
    GenericData_t data1 = {100, 'a'};
    GenericData_t data2 = {200, 'z'};
    genericStructPrint((void *) &data1);
    genericStructPrint((void *) &data2);
    return 0;
}
```

As you can see from the above example the `void *` is a very efficient way to send generic data to a function. As opposed to function overloading practiced in C++ this method can be used where code flexibility is of the utmost importance.

## Conclusion

From the above example you can see, we slightly modified the output/working of a function by supplying two different arguments to the same function. 

In this way we reduce redundant code and improve readability.

In the next tutorial, we shall task more about delaying tasks and delaying tasks until a certain time. These functions would help us give more control over Task based timing actions and creating asynchronous tasks.

**Meanwhile, have fun and keep learning!**