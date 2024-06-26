# roth specification

## Instructions

|   Name   | Description                                                                               |
| :------: | ----------------------------------------------------------------------------------------- |
|   `+`    | Add the top two entries on the stack together                                             |
|   `-`    | Subtract the top two entries on the stack                                                 |
|   `*`    | Multiply the top two entries on the stack                                                 |
|   `/`    | Divide the top two entries on the stack                                                   |
|   `=`    | Check the top two entries on the stack for equality                                       |
|   `<`    | Check the top two entries on the stack for equality                                       |
|   `>`    | Check the top two entries on the stack for equality                                       |
|   `<=`   | Check the top two entries on the stack for equality                                       |
|   `>=`   | Check the top two entries on the stack for equality                                       |
|  `drop`  | Pop the top of the stack                                                                  |
|  `load`  | Load the constant at the address on top of the stack                                      |
|  `swap`  | Swap the top two entries on the stack                                                     |
|  `tRot`  | Rotate the three top stack elements by wrapping the third element to the top of the stack |
|  `dup`   | Duplicate the top of the stack                                                            |
|  `dDup`  | Duplicate the second element on the stack                                                 |
|  `tDup`  | Duplicate the third element on the stack                                                  |
|  `jump`  | Jump to the address on the stack                                                          |
|   `if`   | Jump to the first address on the stack if the second element on the stack is not zero     |
|  `!if`   | Jump to the first address on the stack if the second element on the stack is zero         |
| `abort`  | Abort the virtual machine                                                                 |
|  `exit`  | Exit the virtual machine with the exit code on top of the stack                           |
| `panic`  | Panic virtual machine with the message on top of the stack                                |
|   `gc`   | Run the garbage collector                                                                 |
|   `ln`   | Write a newline to standard output                                                        |
| `input`  | Read a line from standard input                                                           |
| `print`  | Write the string on top of the stack to standard output                                   |
| `~float` | Convert the integer on top of the stack to a float                                        |
|  `~int`  | Convert the float on top of the stack to an integer                                       |

## No verify instructions

|   Name   | Description                                            |
| :------: | ------------------------------------------------------ |
| `%drop`  | Drop an element from the compile-time type stack       |
|  `%int`  | Push the integer type onto the compile-time type stack |
| `%float` | Push the float type onto the compile-time type stack   |
|  `%str`  | Push the string type onto the compile-time type stack  |

## Constants

To push an integer onto the stack just write the integer:

```py
10
```

To push a float onto the stack you have to put a dot into the number to let the parser know it should
be a float instead of an integer:

```py
2.5
```

To push a string onto the stack you have to write the string surrounded by double quotes:

```py
"Hello world!"
```

## Labels

> **Warning** Labels can be unsafe

You create a label like this:

```py
:label_name
```

Do this to load the address of a label:

```py
&label_name
```

To jump to a label do this:

```py
@label_name
```

## Comments

Comments are everything in a line that is preceded by a hashtag `#`:

```py
# this is a comment
```
