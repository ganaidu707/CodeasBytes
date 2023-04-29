#include<stdio.h>

/* Swapping of two numbers:
1. using a third vairable 
2. without using it */
int main() {
    int a = 10, b = 20, temp = 0;

    // 1st method
    temp = a;
    a = b;
    b = temp;
    printf("after swapping 1st method a: %d, b: %d\n", a, b);
    a = 10, b = 20;

    // 2nd method
    a = a + b; //a = 30 & b = 20
    b = a - b; // a = 30 & b = 10
    a = a - b; //a = 20 & b = 10;
    printf("after swapping 2nd method a: %d, b: %d\n", a, b);
    return 0;
}