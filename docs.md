## Writing unittest in rust

- the following is a basic example to write tests in  rust


``` rust

fn add_numbers(x:i32,y:i32)->i32 {
    x+y
}
[cfg(test)]
mod Test{ 


use super::*;
#[test]
fn test_something(){
 let results=add_numbers(3,4)

 assert_eq!(results,7)
}

}

```


- The code above is a simple example of how to add tests lets break it step by step

- we have function that takes to integers
and computes the results  which is an integer

- [cfg(test)] stands for configuration 
means do not run the code below unless you
do are running the rests

- super brings the function to scope

- we use #[test] to mark it as tests


- assert_eq macro is used to compare between the two values since the sides are equil the tests pass
- to run the tests do cargo test 

