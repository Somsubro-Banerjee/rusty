// we will be learning about structs here
/*
structs in ruse is similar to calss in java or struct in c++ and c
defining a struct looks like this

    struct Person {
        name: String,
        age: u8,
    }

    struct User {
        acitive: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

        now to use a struct we have to assign a solod value to it like this:

        fn main() {
            let user1 = User {
                email: String::from("somsubrob@gmail.com"),
                username: String::from("somsubrob"),
                active: true,
                sign_in_count: 1,
            };                  // note that while assignig values we do not use '=' sign but just use  ':'
        }


        now to access the value of the struct we use a simple dot notation like this:
        println!("{}", user1.email);
        in order to change the value of the struce we must ensure that the value is mutable

        so here as the value of the string user email is mutable we can directly change the value of the string like this:

        user1.email = String::from("somsubrobanerjee@gmail.com")


    * using structs in functions:

    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        } // here we are directly returning the value of struct User which is the struct we defined above
    }

    the aove implementation of returning a struct is a bit tiedious as we have to assign the value of the struct to a variable
    luckly rust has a shortcut to do this:
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
    

    * creating instance from a struct:

    fn main() {
        let user2 = user {
            email: String::from("abcd@gmail.com"),
            username: user1.email,
            active: user1.active,
            sign_in_count: user1.sign_in_count,
        }
    } // using the dot operator to access the value of the struct we created above


    there is a more quicker way to create a struct instance from a struct
    fn main() {
        let user2 = User{
            email: String::from("abcd@gmai;/com"),
            ..user1 // here we are using the '..' operator to copy the value of the struct user1 to the struct user2
        }
        // the .. oerator tells the compiler to copy the rest of the values of the struct user1 to the struct user2
        // * note that the ..user1 must come at last to specify this
    }
 */



fn main() {
    println!("Hello, world!");
}
