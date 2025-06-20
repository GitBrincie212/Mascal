<h1 align="center">Mascal Programming Language</h1>
This was a project where i (McBrincie212) had <u><b>2 weeks</b></u> to make a full interpreter toy programming language written in <b>Rust</b>,
it has a lot of features but the main keypoints are:<br />

- <b>Semicolon And Brace Syntax</b> You separate statements with ``;`` and create "blocks" via ``{...}``
- <b>Simple Arithmetic</b> you can use ``+``, ``-``, ``*``, ``%``, ``/`` and even ``^`` just like you are used to other
languages
- <b>Control-Flow</b> Handle conditional logic with ``IF``, ``ELIF``, ``ELSE`` and for loops handle it with 
``WHILE``, ``FOR``. Chain them together and you can write anything!
- <b>Functions</b> You can define your own functions which can be used in the main program by invoking it. You can
pass parameters to your own function and even mutate the actual variable via that parameter
- <b>Recursion</b> Are you tired of regular loops like ``WHILE`` or ``FOR``? Then you can also recurse in functions
- <b>Multitude Of Builtins</b> Want more complex arithmetic? Then use ``SQRT``, ``LOG``, ``ABS``, ``SIN``, ``MAX``... etc 
functions, want to read and write user input? Use ``READ`` and ``WRITE`` functions respectively, want to work with arrays? 
Use ``PUSH``, ``REMOVE``, ``SHIFT``, ``LEN``... etc methods, you got a multitude of options at your disposal, no limits!
- <b>Multitude Of Operations!</b> Wish you could concat strings together? Or even add arrays together? You can!
- <b>Dynamic Arrays</b> You can initialize dynamic arrays with a starting size to allocate to memory and mutate the contents
of the array by adding/removing and modifying values inside it
- <b>Static Arrays</b> You want a more optimized version of a dynamic array and no array size mutations? Static arrays come
to the rescue, allowing you to specify a specific size and no matter what, all values will have to follow that user-defined size
- <b>Types</b> You can get an atomic type of any value by using ``TYPEOF`` followed by the value, you can then store it
in a variable (with the type of ``TYPE``) and compare types together 
- <b>Type Casting</b> You can cast any value into a specific type (as long as it's applicable to that value)
- <b>Static Types</b> Initialize variables with a specific atomic type such as an integer, a float... etc. And let the
interpreter carry things from there by enforcing this static typing pattern for this variable
- <b>Dynamic Type</b> Want an easier handling of variables without doing any boilerplate? With the dynamic type, you 
can change a variable with any value type, and you are not restricted to just one type (tho you still have to handle
arrays the same way)
- <b>Throwing Errors</b> You can throw your own errors via ``THROW`` with any user-accessible error
- <b>Initial Values</b> Forgot any variables to initialize on the program? Well, you can initialize them in
the variables block in an effort not to bloat the program
- <b>Constants</b> You want to prevent any modification of a variable? Then feel free to initialize a constant with ``const``
- <b>Nullables</b> By default, all variables cannot be nor contain **``NULl``**, however with ``?`` you can
allow that to happen

Creating a "Hello World" program is straightforward as creating a file with the extension ``.mascal`` and dropping this code
```pest
DEFINE_PROGRAM {
    IMPLEMENTATION {
        WRITE("Hello World");
    }
}
```

This language is in early development and thus needs more testing as well as optimizations to hit the production stage 
(tho this language won't be taken as seriously as my other projects which have higher priority, even if it's an impressive 
achievement)