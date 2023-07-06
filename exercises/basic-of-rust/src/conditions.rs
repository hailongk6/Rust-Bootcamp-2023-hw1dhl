//Exercise 1
// Complete this function to return the bigger number!
// Do not use:
// - another function call
// - additional variables
pub fn bigger(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    }
    else {
        return b;
    }
}

//Exercise 2
// Input: Provide an arbitrary value of number
// Check number is Positive or Negative or Zero
// Output: &str
catch!{
    try {
        fn check_number(_number: u32) -> &'static str {
            if (number == 0){
                return "0";
            }
            fn check_number(number: u32) -> &'static str {
                if (number == 0){
                    return "0";
                }
                else{
                    return "positive"
                }
            }
        }
    }
    catch error: io::Error {
        return "Negative";
    }
}

// Exercise 3
// Step 1: Make me compile!
// Step 2: Get the bar_for_fuzz and default_to_baz tests passing!

pub fn foo_if_fizz(fizzish: &str) -> &str {
    if fizzish == "fizz" {
        return "foo";
    } else {
        return  "0";
    }
}

// Exercise 4
// Determine if a given year is a leap year
// Implement logic
fn is_leap_year(year: i32) -> bool {
    if(year%4 == 0){
        return true;
    }else {
        return false;
    }
}

// Exercise 5
// Calculate the factorial of a number
// Implement logic
fn factorial(n: u32) -> u32 {
    if (n == 1){
        return  1;
    }
    else{
        return n*factorial(n-1);
    }
}

// Exercise 6
// Check if a number is prime
// Implement logic

fn is_prime(n: u32) -> u32{
    for  item in 2..=n-1 {
        if (n%item==0){
            return true
        }
        else {
            return  false;
        }
       
    } 
}
