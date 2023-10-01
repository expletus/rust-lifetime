# Overview 

Library Crate created to learn **[Crust of Rust: Lifetime Annotation](https://www.youtube.com/watch?v=rAl-9HwD858)**

- Why lifetime? - to avoid  dangling reference, Rust needs to know how long is the reference valid (underlying memory has not been free'd)
- Rust allows only one owner for a value but multiple borrowers (immutable references).
- Lifetime enforces a piece of memory is still valid for a reference. Memory doesn't get cleaned up before a reference can use it. 
- Rust compiler automatically assigns a lifetime parameter to each **reference param** to a function 

```
fn some_func(param1: &str, param2: &str, param3: Vec<64>) -> String // Gets converted to:
fn some_func<'a, 'b>(param1: &'a str, param2: &'b str, param3: Vec<64>) -> String // here it's okay to assign lifetime and also doesn't matter since return type isn't reference
But if we return a reference it's problem
fn some_func<'a, 'b>(param1: &'a str, param2: &'b, param3: Vec<64>) -> &str 
// It's a problem now because compiler won't know which lifetime param to apply to output 'a or 'b if either can be returned.

fn some_func<'a, 'b: 'a>(param1: &'a str, param2: &'b, param3: Vec<64>) -> &str // Lifetime subtyping 'b : 'a means b has to live at least as long a but its a little verbose
But the best fix is to use the same lifetime for all the params - hence the function becomes safe for all calling code in the future.
```

#### Below Example is valid if we don't assign reference of b to a
```
fn main() {
    let a;
    {
        let b = String::from("Some Value");
        a = b;
    }
    println!("{}",a);
}
```
#### Below Example Compiler knows that parameter isn't passed in, so reference is from memory allocated within the method which will get cleaned up when it goes out of scope. 
```
fn main() {

}
fn get_int_ref() -> &i32 {
    let a= 1;
    &a //Compiler complains
}
```
#### Multiple reference param as input and retrun type is reference
- Fix is to either subtype lifetime 'b : 'a -> verbose
- Another fix is to explicitly assign same lifetime param to all the params and return.
- Fix is required even if calling code is safe i.e. param_1 and param_2 are being passed in from same scope! This is because rust has to make function definition safe, in the future it could be used from a calling code where param_1 and param_2 have different lifetimes.
- Explicitly assigning the same lifetime is preferred. Rust uses the minimum of the passed in lifetimes.
```
fn get_str_ref(param_1 :&str, param_2: &str) -> &str { //Will complain as Compiler automatically assigns 'a to param_1 and 'b to param_2 and won't know which will be returned!!!
    if param_1 > param_2 {
        param_1
    }
    param_2

To Fix :

fn main() {
    let str = get_str_ref("abc", "abcd");
    println!("{}",str);
}
fn get_str_ref<'a>(param_1 :&'a str, param_2: &'a str) -> &'a str {  
    if param_1 > param_2 {
        return param_1
    }
    
    param_2
}
}
```


```
//param_ is not a reference, lifetime don't apply. Also its invalid because param_1 goes out of scope when function returns so reference to it is invalid anyway
fn test<'a>(param_1: Vec<f64>) -> &'a Vec<f64> { 
    &param_1
}
```

- Okay to ignore providing Lifetime parameter to a reference param which has no chance of being returned? Not really!
```
fn get_str_ref(param_1 :&str, param_2: &str) -> &str {
    if param_1 > param_2 {
        return param_1
    }
    println!("{} should be returned but for lifetime experiment we'll return {}", param_2, param_1);
    param_1
} 
```