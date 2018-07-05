---
title: "Function Pointers"
categories:
  - freertos
tags:
  - c

# Test with --future
# date: 2018-06-09

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

Adding a function (pointer) to a function argument

## Introduction

### What is a function pointer?

Just as you can have a pointer to an integer or other data type, a variable can also be a pointer to a function.

Example of Function Pointers declaration

``` c
//1. function pointer, returning void, takes int as an argument
void (*v)(int);
void v(int test); //function

//2. function pointer, return an integer, takes 2 ints as arguments
int (*v)(int, int);
int v(int test1, int test2);

//3. function pointer, returns a char pointer, takes 2 ints as arguments
char *(*v)(int, int);
char *v(int test1, int test2);

//4. function pointer, returns a char pointer, takes an int and void pointer as arguments
char *(*v)(int, void *)
char *v(int test1, void *test2);
```

Take a look at this [example](/assets/code/2018-06-09-function-pointer/basicExample/main.c) to understand a simple implementation of the above declaration

### Why is a function pointer important?

As you can see in the example and declarations above, there isn't much scope in the normal usage for function pointers.

However, where function pointers really shine is as **callback functions**.

**Example**
You want to print a generic message inside another function

``` c
//NOTE, This function takes in an int and prints the output
void genericFunction(void (*printFunction)(int), int data);

void writeYourOwnPrintFunction(int data);
void yourNewPrintFunction(int newData);

int main()
{
  genericFunction(writeYourOwnPrintFunction, 10);
  genericFunction(yourNewPrintFunction, 10);

  return 0;
}

void genericFunction(void (*printFunction)(int), int data)
{
  printFunction(data);
}

void writeYourOwnPrintFunction(int data)
{
  printf("Hello Data: %d\n", data);
}

void yourNewPrintFunction(int newData)
{
  printf("New Function: %d\n", newData);
}
```

Download this example [here](/assets/code/2018-06-09-function-pointer/addFunctionToFunctionArgument/main.c)

## Conclusion

As you can see Function Pointers are easy to understand and implement.

Function Pointers are used as callback functions in the FreeRTOS kernel very frequently hence we need to have a good grasp of how it works.
