# Decoder

> research for .class, oat, elf & oat format file for compiler.

## Library

### Dalvik

Documents:

 - [Creating a Dalvik parser in Rust](https://superanalyzer.rocks/2016/10/18/dalvik-parser-1)
 - [Reverse engineering and penetration testing on Android apps: my own list of tools](https://www.andreafortuna.org/2019/07/18/reverse-engineering-and-penetration-testing-on-android-apps-my-own-list-of-tools/)

Library:

 - [https://github.com/mdeg/dexparser](https://github.com/mdeg/dexparser) A Rust library for parsing Android's DEX file format with parser combinators.
 - [https://github.com/letmutx/dex-parser](https://github.com/letmutx/dex-parser)  Rust parser for Android's dex format. 
 - [https://github.com/SUPERAndroidAnalyzer/dalvik](https://github.com/SUPERAndroidAnalyzer/dalvik)  Dalvik parser in pure Rust. 

Converter
 
  - [dex2jar](https://github.com/pxb1988/dex2jar)  Tools to work with android .dex and java .class files.

### ELF

 - [Falcon](https://github.com/falconre/falcon)  is a formal binary analysis framework in Rust.
 - [panopticon](https://gitlab.com/p8n/panopticon) A libre program analysis library for machine code.
 - [libgoblin](https://github.com/m4b/goblin) An impish, cross-platform binary parsing crate, written in Rust.
 - [https://github.com/aep/elfkit](https://github.com/aep/elfkit) An elf read and manipulation library in pure Rust (written from scratch, no bfd, no gnu code, no license infections), intended to be used in binary manipulation utils such as strip, chrpath, objcopy and ld. The end goal is to build a well designed library that facilitates all sorts of binary manipulation magic.

### Core

[panopticon](https://gitlab.com/p8n/panopticon) is a libre program analysis library for machine code.

 - Disassemble AMD64/x86, AVR, MOS 6502 and MIPS (WIP).
 - Open PE and ELF files.
 - Translate code to RREIL, a reverse engineering focused intermediate language in Single Static Assignment form (including memory operations).
 - Construct control flow graphs and compute liveness information for each basic block.
 - Interpolate stack pointer values using Guilfanov's method.
 - Compute function summaries and substitute call sites with them.
 - Run Abstract Interpretation analysis with fixed cardinality sets, strided intervals and Value Sets.
 - Propagate constants, recover local variables and resolve indirect jumps.

[Falcon](https://github.com/falconre/falcon) is a formal binary analysis framework in Rust.

 - Expression-based IL with strong influences from RREIL and Binary Ninja's LLIL.
 - Semantically-equivalent binary translators for 32/64-bit x86, Mips, and Mipsel.
 - Lifters for ELF and PE via goblin.
 - Fixed-point engine for data-flow analysis and abstract interpretation.
 - Performant memory models for analysis.
 - A concrete executor over Falcon IL.

### Proguard

 - [Rust Proguard Parser](https://github.com/getsentry/rust-proguard)

## Documents

 - linux -> [Smallest x86 ELF Hello World](http://timelessname.com/elfbin/)
 - [ELF Hello World Tutorial](https://cirosantilli.com/elf-hello-world)

Video:

 - [The Teensy ELF Executable](https://www.muppetlabs.com/~breadbox/software/tiny/techtalk.html)

### Videos Tools 

`script`, `scriptreplay`, `recordmydesktop`, `audacity`, `pitivi`, and `ffmpeg`. 

## Specification

 - [Dalvik Executable format](https://source.android.com/devices/tech/dalvik/dex-format)
 

License
---

[![Phodal's Idea](http://brand.phodal.com/shields/idea-small.svg)](http://ideas.phodal.com/)

@ 2020 A [Phodal Huang](https://www.phodal.com)'s [Idea](http://github.com/phodal/ideas).  This code is distributed under the MPL license. See `LICENSE` in this directory.
