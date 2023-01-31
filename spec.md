# roth specification

## Instructions

|   Name   | Description                                                                           |
| :------: | ------------------------------------------------------------------------------------- |
|   `+`    | Add the top two entries on the stack together                                         |
|   `-`    | Subtract the top two entries on the stack                                             |
|   `*`    | Multiply the top two entries on the stack                                             |
|   `/`    | Divide the top two entries on the stack                                               |
|   `=`    | Check the top two entries on the stack for equality                                   |
|   `<`    | Check the top two entries on the stack for equality                                   |
|   `>`    | Check the top two entries on the stack for equality                                   |
|   `<=`   | Check the top two entries on the stack for equality                                   |
|   `>=`   | Check the top two entries on the stack for equality                                   |
|  `drop`  | Pop the top of the stack                                                              |
|  `ldc`   | Load the constant at the address on top of the stack                                  |
|  `swp`   | Swap the top two entries on the  stack                                                |
|  `dup`   | Duplicate the top of the stack                                                        |
|   `if`   | Jump to the first address on the stack if the second element on the stack is not zero |
|  `!if`   | Jump to the first address on the stack if the second element on the stack is zero     |
| `abort`  | Abort the virtual machine                                                             |
|  `exit`  | Exit the virtual machine with the exit code on top of the stack                       |
| `panic`  | Panic virtual machine with the message on top of the stack                            |
|   `ln`   | Write a newline to standard output                                                    |
| `input`  | Read a line from standard input                                                       |
| `print`  | Write the string on top of the stack to standard output                               |
| `~float` | Convert the integer on top of the stack to a float                                    |
|  `~int`  | Convert the float on top of the stack to an integer                                   |

## Constants

To push an integer onto the stack just write the integer.

To push a float onto the stack you have to put a dot into the number to let the parser know it should
be a float instead of an integer.

To push a string onto the stack you have to write the string surrounded by double quotes.
