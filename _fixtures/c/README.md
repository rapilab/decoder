```
gcc -o hello hello.c
```

macOS requirements:

```
brew install binutils
```

run

```
readelf -h chello
```

macOS:

```
gobjdump -p hello

hello:     file format mach-o-x86-64
 MACH-O header:
   magic:      0xfeedfacf
   cputype:    0x1000007 (X86_64)
   cpusubtype: 0x3 (X86_ALL)
   filetype:   0x2
   ncmds:      0x10
   sizeocmds:  0x558
   flags:      0x200085
   version:    2
```