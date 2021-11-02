fn checked_addition(x:&[u32]) -> Option<u32> {
    let mut sum=0;
    for i in x {
        sum += i;
     }// end of for loop

    if sum > u32::MAX {
        None
    }else {
        Some(sum)
    }
}// end of fn another_function


fn main() {
    let v = vec![1,2,3,4,5,6,7,8,9,10];
    match checked_addition(&v) {
        None => println!("integer addtion failed! overflow occured"),
        Some(summation) => {
            println!("Sum is {}", summation)
        },   
    }
    
}