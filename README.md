# stk
## Brainf*ck derivative, but smaller

## Commands
```c
|= // increment current pointer value
= // decrement current pointer value

> // increment current pointer by 1
|> // decrement current pointer by 1

. // print value of current pointer
|. // take current input from stdin

~ // start/end loop

anything else // comment
```

## Examples
```c
// print a character
|= |= |= |= |= |= ~|= |= |= |= |= |=~ . // print '$'
```