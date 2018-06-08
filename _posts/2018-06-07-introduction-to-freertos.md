---
title: "Introduction to FreeRTOS"
categories:
  - freertos
tags:
  - c

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

What is FreeRTOS and Why is it important?

## Website and Things to download

[Free FreeRTOS Books and Code Examples](https://www.freertos.org/Documentation/RTOS_book.html)

[Supported Microcontrollers](https://www.freertos.org/RTOS_ports.html)

## Who is this tutorial series for?

RTOS (Real-Time Operating Systems) is an advanced level concept building upon previously known embedded knowledge.

You should already be familiar with:
- 8 bit microcontroller peripheral interfacing (cannot run freeRTOS but programming concepts remain the same)
- C Programming, since the entire FreeRTOS kernel is written in C (Certain advanced topics will be covered by me)

## What is FreeRTOS?

FreeRTOS is a real-time kernel created for microcontrollers and small microprocessors. What that means is that FreeRTOS allows devices with a small memory and single processing core to perform multi-tasking operation (mainly through a process known as time-splicing).

## What is Time Splicing

For various different polling operations requiring a specific amount of time to perform a task very often programmers implement a `delay()` function.
However the down side to doing this is that no other ***process*** or ***operation*** can occur till this delay loop completes.

This is a huge waste of resources, since the time spent during this delay can be used to perform other time critical tasks.

***Time splicing*** is a method that allows multiple different *tasks* or *operations* to run together by sub-dividing the time spent doing each operation.

## Example

Say you need to read an Analog Input from a device and subsequently drive a motor. However you also want to send an SMS using a GSM Module every 20 seconds.

### Implementation using Delay loops

Using this approach, we would first read the Analog Input and signal the motor, then send the SMS and delay for 20 seconds.
The next loop cycle (reading Analog Input again) would happen after 20 whole seconds.

**This is an extremely bad design**
{: .notice--danger}

### Implementation using the commonly used millis() function (On the arduino)

Using this approach, we would read the Analog Input and signal the motor, we send the SMS and note the current `millis()` time.
The Analog Input would happen throughout each loop however the SMS would only get sent again if `millis() - previousTime > 20 seconds`.

This approach is a better design however it still involves manual labour, in the sense that we need to write a bunch of `if-else` statements for every clause that we would need. For a more complication design this would soon become very tedious.

### Implementation using FreeRTOS

Using this approach, we would create two functions, `analogMotorTask()` and a `sendSMS()` Function.

The analogMotorTask function would look something like this

``` c
analogMotorTask()
{
  // This occurs only once
  // Initialization of Analog and Motor here
  initAnalogPin();
  initMotor();

  // This occurs once every second
  while(1)
  {
    // Read Analog Input
    analogRead();
    // Signal the Motor
    signalMotor();
    // Block AnalogMotorTask function for 1 second
    vTaskDelay(1000);
  }
}
```

The sendSMS function would look something like this

``` c
sendSMS()
{
  // This occurs only once
  // Initialization of GSM here
  initGSM();

  // This will occur every 20 seconds
  while(1)
  {
    // Send SMS
    sendingSMS();
    // Block sendSMS function for 20 seconds
    vTaskDelay(20000);
  }
}
```

As you can see, Each function is an entire program in itself.

However, instead of blocking the entire processor the function blocks itself for a set amount of time. During this time period other functions can run.

FreeRTOS makes this very easy since the entire operation is taken care of by a scheduler that takes care of this `time splicing` for us.

Moreover, as long as the memory of the microcontroller allows us, we can keep adding functions like these and take advantagae of multi-tasking using FreeRTOS easily.

**NOTE: This is an example of how FreeRTOS tasks are implemented, however to get it to work you need to do some additional steps which will be highlighted in following posts.**
{: .notice}
