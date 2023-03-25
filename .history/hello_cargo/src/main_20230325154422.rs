// use std::io;
// use std::env;

fn main() {
    println!("Hello, world!");
}

// Hi THis is message
// Hi THis is message
// Hi THis is message
// Hi THis is message
// Hi THis is message
// Hi THis is message
// Hi THis is message

// trying out something new

// everythin till chapter 4 is below this line

// dangling refrences


// fn main() {
//     let ref_to_nothing = dangling_ref();
// }

// fn dangling_ref() -> &String {
//     let s: String = String::from("Hellow worls");

//     &s
// }

/*
This is a classic example of a dangling reference.
here we see that we are callig a funtion called dangling_refernces() that returns a reference to a string.

looking closely ot the funciton dangling_refrence() we see that the return type is a reference to a string.
and when returning it the refrence to the string will be lost as after the function call the string is no longer in scope.
so the refrence of the stiring s points to NULL so this becomes a dangling refrence.

 */



// understanding mutability with scopes and refrences 
// fn main() {
//     let mut s = String::from("hello");

//     // let r1 = &mut s;         this is an error since we cannot borrow a mutable string more than once at a time
//     // let r2 = &mut s;         to achieve successful borrowing of mutable srtings we need a change of scopes
//                                 //like adding a println!() statement to the end of each of the following lines
//                                 //or by just putting one of he borrowed variable into a curly bracket like so:
//                                 //{let r1 = &mut s;} let r2 = &mut s;
    
//     // println!("{}, {}", r1, r2);
// }




// ownership and borrowing

// fn main() {
//     let s = String::from("Hello");

//     takes_ownership(&s);
//     println!("{}", s);
//     let x = 10;
//     makes_copy(x);
//     println!("{}", x);
    
// }

// fn makes_copy(x: i32) {
//     println!("{}", x);
// }

// fn takes_ownership(s: &String) {
//     println!("{}", s);
// }

/*
Understanding Ownership and Borrowing:
to understand the following we need to understand what scopes are and how they work.
Scopes are the areas of code where variables are accessible. scopes are defined by curly braces
in the above main function we have decared a variale s of tyope string inside the main scope,
and then we have called the function takes_ownership on the variable s
since String does not implement the Copy trait the function takes_ownership takes the ownership of s and moves it to the function
so when this happens we are not able to use the value of s anymore as the ownership has transferred.

in the second variable clled x the type of the variable is i32 or signed 32 bit integer
and the function makes_copy takes the value of x and copies it to the function, here the value of x is copied to the function
and the ownership of x is not transferred to the function. because the type i32 implements the Copy trait.

it is important to know what copy trait is and how it works.
most of the datatypes of the rust language implement the Copy trait.
like:
* all integers signed and unsigned : i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
* all floating point numbers : f32, f64
* all boolean types : bool
* all character types : char
* all tuples as long as the tuples are of same type like: (i32, f64, bool) and not like: (i32, String)
* String do not implement the Copy trait becauser the String object allocates memory on the heap. and not on the stack.

how ever to make use of the varialbe s in the function takes_ownership we have to use the &s syntax.
the &s allows the rust compiler to know that s is a reference to a String object and so it is borrowed and not owned.
borrowing is a way to share ownership of data with another piece of code.
on borrowing the pointer to the data is shared and not copied.

 */



// fn main() {
    
//     let args: Vec<String> = env::args().collect();
//     println!("{:?}", args);


    // println!("My first Rust program!");
    // println!("This program will print fibinacci numbers in the console.");
    // println!("Enter the number of fibinacci numbers you want to print.");
    // let mut n = String::new(); // declaring a mutable variable called n of String type

    // io::stdin() // returns a reference to the standard input stream
    //     .read_line(&mut n)  // read_line() takes a reference to a mutable variable
    //     .expect("Failed to read line"); // expect() is used to handle errors
    
    // let n: u32 = n.trim().parse().expect("Please type a number!");  // trim() removes whitespace from the start and end of the string
   
    // println!("The fibinacci numbers are:");
    // for i in 0..n {                                             // for loop is used to print fibinacci numbers
    //     println!("{}", fibonaacci(i));
    // }
// }

// fn fibonaacci(n: u32) -> u32 {                               // function fibonaacci is used to print fibinacci numbers and returns the fibinacci number, arrrow head denotes the retuen type of the function
//     if n == 0 {
//         return 0;
//     } else if n == 1 {
//         return 1;
//     } else {
//         return fibonaacci(n - 1) + fibonaacci(n - 2);
//     }
// }

// CLI argument program to print my name
// fn main() {
//     let args: Vec<String> = std::env::args().collect();
//     println!("{}", args[0]);
// }


/*
Rust is a statically typed language.
therefore, the compiler will check the types of the variables and functions.

There are mny data types (boith signed and unsigned) in rust like:
unsigned: u8, u16, u32, u64, u128, usize
signed: i8, i16, i32, i64, i128, isize

 * a function looks like this:
 * fn function_name(arg: data_type) -> return_type {
 *  some functionality
 * return value
 * }

* a variable looks like this:
* let variable_name: data_type = value;  // value is optional
we can also declare variables without initializing them. like so:
* let variable_name: data_type;
we can diretly store data into the variable like so:
* let variable_name = value;
we can decalre variables in 2 differnt ways 
* let variable_name: data_type = value;
* let mut variable_name: data_type = value;
the first one is immutable and the second one is mutable.
immutable means: the variable can't be changed after it's been initialized.
mutable means: the variable can be changed after it's been initialized.
whenver we try to use the variables in our program we need to use the mutable keyword.
we mostly user reference to acces the data in the variable. like so:
* let mut variable_name: data_type = value;
* let variable_name: &data_type = &value;
* let variable_name: &mut data_type = &mut value;

when printing a mutable variable we need to use the &mut keyword.
when printing a immutable variable we need to use the & keyword.
like so:
* println!("{}", &variable_name);
here we are printing the value of the variable, by using the & keyword. i.e. we are referencing the value of the variable.
* println!("{}", &mut variable_name);
here we are printing the value of the variable, by using the &mut keyword. i.e. we are referencing the value of the variable. but we are also allowing change of the value of the variable.

* a simple for loop looks like this:
* for i in 0..n {
    * some functionality
    * }
explaination: here we are iterating over the range of 0 to n. which is denoted by the .. operator.
which means for every number i in the range of 0 to n, we will execute the code in the curly braces.
*note we do not use the traditional '>' and '<' operators in rust. instead we use the '..' operator.
also we dont need to decalre the data type of the variable i. rust will figure it out for us.
in a for loop we can use the break and continue keywords. like so:
* for i in 0..n {
    * if i == 5 {
        * break;
        * }
* for i in 0..n {
    * if i == 5 {
        * continue;
        * }

*break keyword: breaks the loop. and jumps out of the loop.
*continue keyword: continues the loop. and jumps to the next iteration. and skips the rest of the code in the loop. 
we can use the break and continue keywords together in a for loop. as so:
* for i in 0..n {
    * if i == 5 {
        * break;
        * }
    * if i == 6 {
        * continue;
        * }

* A while loop looks like this:
* while condition {
    * some functionality
    * }
 example to print numbers from 1 to 10 using a while loop we do:
* let mut i = 1;
* while i <= 10 {
    * println!("{}", i);
    * i += 1;
    * }

    note we do use () after the while keyword. this is because rust comliler will understand that the while loop is a function.

* a Do while loop looks like this:
* do {
    * some functionality
    * } while condition;
example to print numbers from 1 to 10 using a do while loop we do:
* let mut i = 1;
* do {
    * println!("{}", i);
    * i += 1;
    * } while i <= 10;

    note we do use () after the do keyword. this is because rust comliler will understand that the do while loop is a function.

* for taking user inpuit we use the io::stdin() function. whis is needed to be imported, using the use keyword. like so:
* use std::io;
now to get the user input into a variable we use the read_line() function. like so:
* let mut n = String::new();
* io::stdin().read_line(&mut n).expect("Failed to read line");
the read_line() function takes a reference to a mutable variable. so we need to use the &mut keyword.
and in case there was on any input detected or the program stops working the .expect() function will handle the error.
as rust doesnot has a garbage collector, we need to make sure that the memory is freed.
.expect() function will handle the error. in such scenarios
* let n: u32 = n.trim().parse().expect("Please type a number!");
her we are declaring a variable n of type u32 (unsigned 32 bit integer) and we are initializing it with the value of the user input.
we are also using the trim() function to remove the whitespace from the start and end of the string.
we are also using the parse() function to convert the string to a number.
* note the user input will always be a string. so we need to convert it to a number. and ot do that we can use the .parse() function.
the .parse() function converts the string to a number.
* let n: u32 = n.parse().expect("Please type a number!");
here in case we enter something other than a number the program will jump to the expect() function and print the error message.

* a match expression looks like this:
    * match variable {
        * pattern1 => {
            * some functionality
            * }
match is a keyword that is used to match a value to a pattern. it is imported from the std::ops module.
that includes the match keyword.
* match variable {
    * pattern1 => {
        * some functionality
        * }
    * pattern2 => {
        * some functionality
        * }

for example lets use the match an expression to print the number of days in a month.
* let month = String::from("January");
* let mut months = match month.as_str() {
    * "January" | "March" | "May" | "July" | "August" | "October" | "December" => 31,
    * "April" | "June" | "September" | "November" => 30,
    * "February" => {
        * let is_leap_year = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
        * if is_leap_year {
            * 29
            * } else {
                * 28
                * }
                * }
here we are declaring a month variable and initilizing it to the string "January".
we are also declaring a mutable months variable and initializing it to the value 31.
her the string months will store the value of the month if it is one of the months with 31 days.
.as_str() function is used to convert the string to a string slice.
a string slice is a reference to a string. which means we can use the string slice to access the string.

*this is how a CLI program looks like:
* fn main() {
    * let args: Vec<String> = env::args().collect();
    * let config = Config::new(&args);
    * println!("{:?}", config);
    * }
here we are declaring a vector of strings called args.
we are using the env::args() function to get the arguments passed to the program.
these areguments can be passed to the program using the command line. like so:
* ./my_program --option1 value1 --option2 value2
here we are passing the options and values to the program.
we are using the collect() function to convert the iterator to a vector.
an iterator is an object that iterates over a collection.
the collect() function takes a closure as an argument.
closure means a function that takes no arguments and returns a value.
the collect() function will take the closure and execute it for each element in the iterator.
in this case the closure will take the element and print it.
* the Config struct is a struct that holds the values of the options.
Config is a struct that holds the values of the options. 
structs are used to create custom data types. its similar to a class in java. or a structure in c /c ++
then we can use the struct to create a custom data types.
finally the println!() function is used to print the values of the options.



 */