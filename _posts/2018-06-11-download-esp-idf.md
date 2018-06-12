---
title: "Download and Setup ESP-IDF"
categories:
  - esp-idf
tags:
  - windows

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
This tutorial is for downloading and setting up the ESP-IDF with the windows operating system.

## Important Links

[ESP-IDF ReadTheDocs](http://esp-idf.readthedocs.io/en/latest/)

[Get Toolchain for Windows](http://esp-idf.readthedocs.io/en/latest/get-started/windows-setup.html)

### Steps for Linux and Mac

[Linux Setup Instructions](http://esp-idf.readthedocs.io/en/latest/get-started/linux-setup.html)

[Mac Setup Instructions](http://esp-idf.readthedocs.io/en/latest/get-started/macos-setup.html)

## Steps

1. Get the toolchain
2. Get the ESP-IDF Core Packages
3. Compiling and running a sample program

## Getting the Toolchain

[Download the Toolchain from here](https://dl.espressif.com/dl/esp32_win32_msys2_environment_and_toolchain-20180110.zip)

**If link is broken, please do go to _Get Toolchain for Windows_ in the _Important Links_ above**
{: .notice--warning}

1. Unzip this to a folder of your choice, default taken as: `C:\`
2. Pin the `C:\mysys32\mingw32.exe` terminal program to your taskbar, since we will need to use this rather frequently to compile, flash and monitor the programs

## Getting the ESP-IDF Core Packages

``` bash
git clone --recursive https://github.com/espressif/esp-idf.git
```
**Make sure you use the `--recursive` option, we need to get all the submodules**
{: .notice--warning}


1. Run the above command in a folder of your choice ex: `D:\GIT\esp-idf`
2. This is used to set up the `IDF_PATH` for your Core Packages

### Setup IDF_PATH Variable to your user profile (Windows)

[Setup IDF_PATH Docs](http://esp-idf.readthedocs.io/en/latest/get-started/index.html#setup-path-to-esp-idf)

**Steps for Windows, Mac and Linux are different, so do check the above link**
{: .notice--warning}

User Scripts are contained in `C:\msys32\etc\profile.d\` directory.

1. Create a new script file in `C:\msys32\etc\profile.d\` and name it `export_idf_path.sh`.
2. Find your ESP-IDF directory, ex: `D:\GIT\esp-idf`
3. Add the `export` command to the script file
``` bash
# Forward slashes are important (Don`t add this comment to file)
export IDF_PATH="D:/GIT/esp-idf"
```
4. Save the script
5. Check if `IDF_PATH` exists by typing
``` bash
printenv IDF_PATH
```

**Make sure use forward slash in the script file instead of backslash, which is common in windows**
{: .notice--warning}

## Compiling and Running a sample program

[Test Program Github Link](https://github.com/espressif/esp-idf/tree/master/examples/get-started/blink)

For Users that have downloaded the esp-idf repository after following the above steps, navigation to the get-started -> blink example

## Steps

1. Copy the entire blink example to another folder (of your choice) ex: `D:\esp32Programs\Tutorials`
2. Open your `MYSYS2 MINGW32 Shell` and navigate to that path (Example given below)
3. Run `make menuconfig` to open menu tree and configure ESP-IDF
4. Run `make` to compile
5. Run `make flash` to flash compiled version to your hardware
6. Run `make monitor` to view monitor output on your console

### Navigate to Path

``` bash
# Example
cd D:
cd esp32Programs
cd Tutorials
cd blink

# We have navigated to the directory
# NOTE, Type ls to see your output
ls
```
You will see output like this

![navigateToPath](/assets/images/2018-06-10/navigateToPath.png)

### Open ESP-IDF menuconfig

``` bash
make menuconfig
```

![makeMenuconfig](/assets/images/2018-06-10/makeMenuconfig.png)

### Set the COM Port of your device

Find your device COM Port
![selectCOMPort](/assets/images/2018-06-10/selectCOMPort.png)

Navigate to Serial Flasher Config (to set the COM port of your device)
![setCOMPort](/assets/images/2018-06-10/setCOMPort.png)

**NOTE: For linux or mac your COM Port might be /dev/ttyUSB0**
{: .notice--warning}

### Set the example Configuration number (OPTIONAL)

Change `Example Configuration` number here `(D2)`
![setBlinkPin](/assets/images/2018-06-10/setBlinkPin.png)

For most devices in ESP32 the internal LED is attached to pin D2 which is pin number 2

Users wishing to test on different pins can change this pin number

See the source code (main.c looks like this)

We can also change `CONFIG_BLINK_GPIO` (which is set inside sdkconfig.h) to the appropriate pin number instead of changing it through `make menuconfig`
``` c
#include "sdkconfig.h"
// The variable CONFIG_BLINK_GPIO has been set through sdkconfig
#define BLINK_GPIO CONFIG_BLINK_GPIO
// OR, change to
#define BLINK_GPIO 2 // (D2), Internal LED Attached to Pin D2
```

### Make the project

`make`

![make](/assets/images/2018-06-10/make.png)

Please note that it can take a significant amount of time to compile all the libraries when running your first `make` command.

### Make flash the image

Start the `make flash` command and you should see it connecting to your device
![makeFlashStart](/assets/images/2018-06-10/makeFlashStart.png)

When `make flash` is completed, this will be your output
![makeFlashEnd](/assets/images/2018-06-10/makeFlashEnd.png)


## Conclusion

After all these steps, you should see your ESP32 internal LED blinking at a constant rate of once per second.

This tutorial has beem about setting up your ESP-IDF with your computer system so the code example has not been explained.

In the next tutorial, I plan to show you how to integrate the ESP-IDF environment with the ECLIPSE IDE for faster prototyping
