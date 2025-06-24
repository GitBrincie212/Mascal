# Getting Started
### Installation
When it comes to using mascal and running code. You have only one option (for now), and that is using the interperter 
(written in [**Rust**](https://www.rust-lang.org)). There are two options to using the interpreter, the first option involves compiling from source
code which will take a time (takes approximately ~40 seconds) via the shell command:
```shell
$ cargo build --release
```
This method assumes you have [**Rust**](https://www.rust-lang.org) installed in your system

---
The second option involves downloading the binaries listed in releases. Which is much more user-friendly, however,
depending on the operating system you are running, this option may be unavailable. Simply download it and run in the shell
```shell
$ ./path/to/binary/mascal-interperterv0.1.0 path/to/my_file.mascal
```
---
### Writing A "Hello World" Program
Now let us get started with writing a simple "Hello World" program as seen in many programming languages. This
guide will assume you are an absolute beginner in this programming language and perhaps even programming in general, so
we are going to go bit by bit

The first thing we are going to define is our **Main Entrypoint**, which mascal will use, this tells the language on
what block to start executing the code
```swift
DEFINE_PROGRAM {
    // ...
}
```
The grayed out ``// ...`` is a comment, used for when you want to put notes on the code and any related stuff.
However, if you run this via the interpreter, it does absolutely nothing. That is because we need to define one more
block that being ``IMPLEMENTATION`` inside the ``DEFINE_PROGRAM`` block
```swift
IMPLEMENTATION {
    // ...
}
```
With that said, let us actually write the code. We will be using one of the various **Builtin Functions**, which
are basically operations that Mascal has implemented for our convenience, that being ``WRITE``. It takes
anything in it and prints it as text, so let us try to write Hello World like so
```swift
WRITE(Hello World);
```
Every line of code (instruction / statement) requires a semicolon at the end (except braces), running this will give us 
an error tho, uh oh... Which is not a good thing
```prolog
ParserError: Expected a comma ',' or closing parenthesis ')' in the function call, but got "World"
AT LINE: 3; STARTING IN CHARACTER POSITION: 59
```
An error tells us that we have something wrong in our program. The wrong part is the very thing we wrote last, that 
being the ``WRITE(Hello World);``. Let us rewrite it into the intended version
```swift
WRITE("Hello World");
```
The quotation marks that surround the ``"``Hello World``"`` are used to capture the characters as text. Without them, it
wouldn't be possible to write the hello world program; programmers refer to these quoted texts as **Strings** (which are
identical to what we have been referring to as text, later on we will be calling them strings as opposed to text). Now
we shall put all the instructions together into:
```swift
DEFINE_PROGRAM {
    IMPLEMENTATION {
        WRITE("Hello World");
    }
}
```
Running this program now will yield the following output in your console (which is called printing):
```
Hello World
```
Tho rambling aside, this wasn't bad and complex of a program to write. Let's shift our gears to a bit more complex program,
and we will be explaining stuff along the way, but before we do that, we shall play around with the ``WRITE`` function more

The ``WRITE`` function can cleanly write multiple values where they are nicely spaced out. Consider this example 
```swift
DEFINE_PROGRAM {
    IMPLEMENTATION {
        WRITE("Hello", "World");
    }
}
```
It prints exactly the same as the original program, but the "Hello World" is split in two strings. That's not all, 
however, it can also print other values such as **numbers**, now numbers are quite special as they have two forms they are
presented as, that being as an **Integer** or as a **Float**. Integers just like learned in math are whole numbers such
as 1, 5, 9..., etc. On the other hand floats are decimals such as 1.234 or 9.291 or even 6.0 (a whole float), and the ``WRITE``
function automatically converts it into a text (String), for example:
```swift
DEFINE_PROGRAM {
    IMPLEMENTATION {
        WRITE("Your article has", 3, "comments and is rated", 4.5, "stars");
    }
}
```
---
### A Bit More Nuanced Program Than Before
With that out of the way, let us make a more sophisticated program. Specifically, a simple addition program where it
adds two numbers together, before we dive into this, let's explore **Variables** and what are they exactly

Variables just like in math have a name, this name is arbitrary and can be anything such as ``my_cool_text``, 
``magic_number``... etc. And with the world ``Variable``, one can conclude that they can change over time, which is
exactly what they are doing and their purpose (i.e representing stuff and holding onto things in order to reuse them
multiple times)

Now let us write this awesome addition program, which will consist of reading the user input. But before we do let us
write out the main entrypoint of the program
```swift
DEFINE_PROGRAM {
    VARIABLES {
        INTEGER {
            x;
            y;
            result;
        }
    }

    IMPLEMENTATION {
        // ...
    }
}
```
**``VARIABLES``** is a block (because of the braces ``{...}``) that lives within ``DEFINE_PROGRAM``, just like 
``IMPLEMENTATION``. In the variable block we can define **variable type block**, which groups related variables into
the same category (for example, whole numbers are integers, and in our program x, y and result are integers). 

However, you have been asking: "How we will read the user input?", the answer to this question is with the builtin 
function ``READ``. Unlike most of its peer builtin functions (including ``WRITE``), this one doesn't accept any traditional
numbers, strings... etc. It specifically requires variables, which we will supply as so
```swift
READ(x, y);
```
Just like ``WRITE``, it can accept an arbitrary number of values; in our case we just need two variables. Showing the 
program together, we shall try it out:
```shell
$ ./path/to/binary/mascal-interperterv0.1.0 path/to/my_file.mascal
> 1
> 2
```
It correctly reads the user input; however, some folks of you may wonder "What happens if I supply something that isn't 
a number", the answer is:
```shell
$ ./path/to/binary/mascal-interperterv0.1.0 path/to/my_file.mascal
> 1
> defintely a number
```
an error is returned:
```prolog
InputError: The user input cannot be parsed as an integer
AT LINE: 3; STARTING IN CHARACTER POSITION: 58
```
Sadly for this type of error, we cannot fix it, as it depends on how the user supplies the input. Maybe lets try
to specify what they should input to the program
```swift
WRITE("Input 2 whole numeric values and witness this glory addition");
READ(x, y);
```
This does nothing on its own to prevent the error, but it does tell the user to input it correctly. Let's
now try to add the two-number together, and store it in ``result``
```swift
result <- x + y;
```
This evaluates the addition and stores it to the ``result`` variable for later use. Which that later use is now
```swift
WRITE("The addition of", x, "plus", y, "equals", result);
```
Putting it all together, we have this program:
```swift
DEFINE_PROGRAM {
    VARIABLES {
        INTEGER {
            x;
            y;
            result;
        }
    }
    
    IMPLEMENTATION {
        WRITE("Input 2 whole numeric values and witness this glory addition");
        READ(x, y);
        result <- x + y;
        WRITE("The addition of", x, "plus", y, "equals", result);
    }
}
```
Running it and giving the inputs ``1`` and ``2``, correctly yields back ``3`` like so
```shell
$ ./path/to/binary/mascal-interperterv0.1.0 path/to/my_file.mascal
> 1
> 2
The addition of 1 plus 2 equals 3
```
Pretty cool, right?

---
### Summary / TLDR;
We learned how to define a main entrypoint for the program and in it write a simple "Hello World" using the simple
builtin function ``WRITE``. We also learnt about errors which indicate something has gone wrong in the program, additionally
we broadened our knowledge and learnt what variables are, how we use the builtin function ``READ`` to read user input, then
learnt how to add 2 numbers together and store it in a variable and finally output back the result in a nice way
---
### Exercise
Write a program that reads three numbers and adds all the three numbers and produces an output that is follows this structure:
```
The addition of [FIRST_NUMBER] plus [SECOND_NUMBER] plus [THIRD_NUMBER] equals [RESULT]
```
Bonus points if you are able to make the addition program subtract three numbers together  
instead of adding them