---
title: "Pointer to Void (Void Pointers)"
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

While **void** in C means *no return type* or *nothing*, **pointer to void** equates to a *pointer to anything* or more specifically a *variable pointing to an address, not knowing it's datatype*.

## I have so many questions!

This basically means that the program doesn't explicitely know the **type** of data you are feeding into a variable or function unless it is explicitely mentioned.

For those who are still confused, take a look at a few code samples given below. It will help you get a much clearer picture.

### Strength of Void Pointers

Void pointers can store data of any type (since we point to the address)

``` c
int i = 10;
char a = 'a';
void *p = &i; //holds the address of int i
p = &a; //now holds the address of char a
```

### Code Example

``` c
typedef enum
{
  UINT8,
  UINT16,
  UINT32
} DataType_t;

void genericPrintFunction(DataType_t dataType, void * data)
{
  if(dataType == UINT8)
  {
    printf("%c\n", (uint8_t) data);
  }
  else if(dataType == UINT16)
  {
    printf("%d\n", (uint16_t) data);
  }
  else if(dataType == UINT32)
  {
    printf("%ld\n", (uint32_t) data);
  }
}

int main()
{
  genericPrintFunction(UINT8, (void *) 'a');
  genericPrintFunction(UINT16, (void *) 137);
  genericPrintFunction(UINT32, (void *) 123456780);

  return 0;
}
```
Download the code [here](/assets/code/2018-06-08-pointer-to-void/genericPrintFunction/main.c)

### Code Explanation

Here we create a *genericPrintFunction()* which is takes in a *DataType_t enum* and a *pointer to void data address* as parameters.

As you can see, we cast the *uint8_t, uint16_t and uint32_t* to *void pointer* inside the function.

When we are reading the data inside the function, we ***type cast*** it appropriately so that the program knows what **type** the data is.

**While compiling the above program, you might get warnings with respect to _uint8_t and uint16_t_ datatypes, this is because according to your architecture, the compiler assumes a 32bit or a 64 bit integer, this is the reason why the compiler doesn't give a warning for _uint32_t or uint64_t_ variables**
{: .notice--warning}

### Architecture based `void *` size
``` c
sizeof(long) == sizeof(void *)
//OR
sizeof(uint32_t) == sizeof(void *)
//For 64 bit machines or compilers
sizeof(long long) == sizeof(void *)
//OR
sizeof(uint64_t) == sizeof(void *)
```

## De-referencing a Void Pointer

**_Void Pointers_ cannot be de-referenced the normal way**
{: .notice--warning}

### De-referencing the normal way

``` c
int a = 10;
int *p = &a;
printf("%d\n", *p); //This works in the normal way
```

### De-referencing with void pointers

Wrong Method
``` c
int a = 10;
void *p = &a;
printf("%d\n", *ptr); //Throws a compile time error
```
We cannot do the above since the variable *ptr* just points to the address, it does not know the type of the variable

Correct Method
``` c
int a = 10;
void *p = &a;
printf("%d\n", (*(int *)ptr) ); //This works
```
We first tell the ptr that it is a int address, we then point to that integer

## Using Structures with Void Pointer

As you have seen above, we have used the generic data types *uint8_t, uint16_t and uint32_t* and we can easily use type casting like *(uint8_t) data* to tell the pointer that it is a *char*. However, with complex datatypes like ***float, double, structs*** this is not so straightforward.

Long story short: Pass by Address, Not by value

### Code Example

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
Download the code [here](/assets/code/2018-06-08-pointer-to-void/genericStructPrint/main.c)

### Code Explanation

The only difference with the previous example and this example is that since it is a complex data type we need to pass the struct into the function parameter by **address rather than by value**

We receive a pointer to the struct inside the function and after type casting we can use it appropriately

## Using Floats or any other complex data type with Void Pointers

Another datatype type does not work with void pointer is float, i.e we cannot type cast `(float) floatData` and expect it to work,
There are different ways to circumvent this issue, given below

We can circumvent this using [Method1](/assets/code/2018-06-08-pointer-to-void/bypassingFloatRestriction/main.c)

We can circumvent this also using method used for structures [Method2](/assets/code/2018-06-08-pointer-to-void/bypassingFloatRestriction2/main.c)

## Conclusion

Void Pointers is extremely advantageous in creating generic functions, where the data type of a variable is not known before hand.

It is due to this nature that it is widely used in the FreeRTOS kernel. Infact becoming proficient in using void pointers will help you in mastering FreeRTOS faster.
