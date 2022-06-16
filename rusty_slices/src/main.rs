// rust slices

/*
Slices are used in rust to refer to a part of an array or a string, slices are used to divide or access any given data into a particular format
lets say for example we need to find the first word in a sentence, we can use slices to do that

the traditional way of finding the first word is to find the spaces and then take the first word
if a semtence doesnot have a space then the first word is the whole sentence
 let sentence: String = String::from("Hello Som!!");
 findFirstWorkds(&sentence);

* fn findFirstWord(s: &String) -> usize {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }
        return s.len();
}

this is a tradiotionnal way of finding the frist space of a sentence and returning the first word of the sentence
heere when the funciton call ton findFirstWord is made with the refrence to the sentence variable
first the senctence is converted into bytes which basically is a vector of bytes or and array of the entire sencence.
then we are using a for loop to iterate over the bytes considering the index and the refrence to the item in the vector and since the 
index and items are a tuple and the iterate function returns a tuple we use enumerate to get the index and the item in the tuple
now when we hit a space character we return the index of the space character
else we return the length of the sentence

this way of finding the first word is not the most efficient way of finding the first word in a sentence
so rust has a simpler way of handeling things using slices

below shows the example of the slice
 */
fn main() {
    let sentence: String = String::from("Hello Som");
    println!("{}", find_first_word(&sentence));
    println!("{}", find_second_word(&sentence));
}

fn find_first_word(s: &String) -> &str{
    
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];                // this is the first word of the sentence from 0 to i where i is the space character
        }
    }
    &s[..]
}

fn find_second_word(s: &String) -> &str {

    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i..];                 // this is the second word of the sentence from i to the end of the sentence
        }
    }
    &s[..]
}