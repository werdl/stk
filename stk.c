#include <stdio.h>

void handle(char c, int *p, int *tape, char **argv, int i) {
    switch (c) {
        case '=':
            tape[*p]--;
            break;
        case '>':
            p++;
            break;
        case '~':
            {
                int loopCount = *p;
                while (loopCount > 0) {
                    
                }
            }
            break;
        case '.':
            putchar(*p);
            break;
        case '|':
            {
                switch (argv[1][i+1]) {
                    case '=':
                        tape[*p]++;
                        break;
                    case '>':
                        p--;
                        break;
                    case '.':
                        *p = getchar();
                        break;
                }
            }
            break;
        
    }
}

int main(int argc, char ** argv) {
    int tape[30000] = {0};
    int *p = tape;
    int i = 0;
    do {
        printf("%c", *argv[1]);
        
    } while (*argv[1]++ && i++);
}