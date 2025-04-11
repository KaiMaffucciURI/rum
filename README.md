# Overview

Authors: Kai Maffucci and Aeyva Rubelo

This took about 20 hours total to complete, with a couple down to analyzing, a couple for designing, and the rest for implementation and debugging. 

Thanks to Dr. Noah Daniels for his rumdump lab from CSC 411, as we used the lab's code for reading the UM binary file and loading it into memory. 

Our UM is fully functional as far as we know, as it was able to run all of the binaries from here: git clone https://github.com/ndaniels/rum-binaries.git

Notable departures:
- The `halt` function is no longer in use and is replaced with a simple process::exit call


# Architecture

## Registers

We have an 8-item long array of u32s called `r` which emulates virtual registers r[0]-r[7]. 

## Memory

A 2d vector (a vector of vectors) of u32s called `m` represents virtual memory segments. Identifiers are the index used to determine the location of the inner vector. We also have a vector of u32s `saved_ids` which stores segment IDs we can re-use and need to store. 

## Main program loop

The program logic is stored within main.rs in a loop. We have a program counter `pc` which points to the current instruction at offset m[0][pc], which is the current/next instruction being executed. In this loop we extract the opcode to determine which operation to perform. Most operations are contained within their own designated functions for that specific operation. These functions are passed mutable references to r, m, etc as needed. The state of the UM is altered by the side effects of each of these functions. So these other functions are all tethered to main, but not to each other. The one execption of that is word.rs, which we use to extract the opcode and register IDs from each instruction word. 

# 50 Million Instructions

 

# Error Codes

| Error Code # | Opcode # | Referenced Operation | Usage/Indication |
|--------------|----------|---------------------|-----------------|
| 11           | 1        | SLOAD (Segment Load) | Program tried to load segment with identifier out of bounds |
| 12           | 2        | SLOAD (Segment Load) | Program tried to load segment with offset out of bounds |
| 21           | 2        | SSTORE (Segment Store) | Program tried to store segment with segment identifier out of bounds |
| 22           | 2        | SSTORE (Segment Store) | Program tried to store segment with offset out of bounds |
| 51           | 5        | DIV (Division)       | Division by zero |
| 121          | 12       | LOADP (Load Program) | Program tried to load program with segment identifier out of bounds |
| 122          | 12       | LOADP (Load Program) | Program tried to load program with offset out of bounds |
| 140          | n/A      | n/A                  | User called non-existent opcode (Usage: 0 < opcode# <= 13) |


# Instruction Set
| Opcode | Operator            | Action                                                                                                                                                               |
|--------|---------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 0      | Conditional Move    | If $r[C] ≠ 0, then $r[A] := $r[B]                                                                                                                                    |
| 1      | Segmented Load      | $r[A] := $m[$r[B]][$r[C]]                                                                                                                                             |
| 2      | Segmented Store     | $m[$r[A]][$r[B]] := $r[C]                                                                                                                                             |
| 3      | Addition            | $r[A] := ($r[B] + $r[C]) \mod 2^{32}$                                                                                                                                |
| 4      | Multiplication      | $r[A] := ($r[B] \times $r[C]) \mod 2^{32}$                                                                                                                           |
| 5      | Division            | $r[A] := ($r[B] \div $r[C])$ (integer division)                                                                                                                       |
| 6      | Bitwise NAND        | $r[A] := \neg($r[B] \land $r[C])$                                                                                                                                     |
| 7      | Halt                | Computation stops                                                                                                                                                     |
| 8      | Map segment         | A new segment is created with a number of words equal to the value in $r[C] . Each word in the new segment is initialized to zero. $r[B]$ is set as its identifier. |
| 9      | Unmap segment       | The segment $m[$r[C]]$ is unmapped.                                                                                                                                  |
| 10     | Output              | The value in $r[C]$ is displayed on the I/O device immediately. Only values from 0 to 255 are allowed.                                                               |
| 11     | Input               | The UM waits for input on the I/O device. When input arrives, $r[C]$ is loaded with the input, which must be a value from 0 to 255. If end of input, $r[C]$ = 1's. |
| 12     | Load Program Segment | $m[$r[B]]$ is duplicated and replaces $m[0]$. Program counter set to $m[0][$r[C]]$. If $r[B]=0$, quick jump.                                                       |
| 13     | Load Value          | Load the value indicated by the remaining 25 bits into $r[A].                                                                                                              |
