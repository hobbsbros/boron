// Autogenerated by the Boron compiler
// Version 0.6.0
// Created on 2022/07/06 at 18:40:31 local time


#include <stdio.h>
#include <stdbool.h>

int main(void) {
int x = 1;
int y = 0;
int z = 0;
int count = 0;
while (count < 10) {
z = x + y;
printf("%d\n", z);
x = y;
y = z;
count = count + 1;
};
}
