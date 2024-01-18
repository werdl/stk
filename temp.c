
#include <stdio.h>

int main() {
char array[30000] = {0};
char *p = array;
(*p)++;
(*p)++;
(*p)++;
(*p)++;
int dup = 2**p;p++;while (2*dup) {dup--;
*p = getchar();
putchar(*p);
}
}