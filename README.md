# ulang

```
#*
This is a multiline comment
*#

# Main function.
local main = fn do
    # Print out the greeting
    println GREETING

    let a = 5
    println a

    # a is mutable
    a += 1
    print a

    # 'const' makes a value constant. It is immutable.
    const B = 6
    println B

    # blocks are expressions. This will print out '5'.
    println do
        2 + 3
    end

    # Function inside a function. Note that 'local' was not used, meaning that it can only be used after declaration. 
    const double_print = fn string1, string2 do
        if string1.len > string2.len do
            println string1
            println string2
        end
    end
end

# 'local' means that it is accessible from anywhere in this scope.
local const GREETING = 'Hey there! You forgot your wallet!'
```
