```
nasm -f elf hello.asm
gcc -o hello hello.o -nostartfiles -nostdlib -nodefaultlibs
strip -s hello
./hello
```