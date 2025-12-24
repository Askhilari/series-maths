fn main() {
    let nth = 20;
    fibonnaci(0,1,nth); // Your will get lucas numbers if you change first two values to 2 and 1.
    tribonnaci(0, 0, 1, nth);
    let numsum = 50;
    let final_sum = sum_of_squares(numsum);
    println!("Sum of squares is: {}", final_sum);
    let root_fiv = fifth_root(final_sum);
    println!("The fifth root is of {} is {}", final_sum, root_fiv);
}


fn fibonnaci(firstno: usize, secondno: usize, n: usize) -> usize{
    let mut first_no = firstno;
    let mut second_no = secondno;
    let mut sum = first_no + second_no;
    
    for _index in 3..n {
        first_no = second_no;
        second_no = sum;
        sum = first_no + second_no;
    }
    println!("{}th fibonnaci number is {sum}", n);
    return sum;
}

fn tribonnaci(firstno: usize, secondno: usize, thirdno: usize, n: usize) -> usize{
    let mut first_no = firstno;
    let mut second_no = secondno;
    let mut third_no = thirdno;
    let mut sum = first_no + second_no + third_no;

    for _index in 4..n{
        first_no = second_no;
        second_no = third_no;
        third_no = sum;
        sum = first_no + second_no + third_no;
    }
    println!("{}th tribonacci number is {sum}", n);
    return sum;
}

fn sum_of_squares(num: i32) -> i32 {
    let mut sum = 0;
    for i in 0..num{
        let odd = 2*i + 1;
        let odd_sq = odd * odd;
        println!("Square of Odd numbers are {}", odd);
        sum = sum + odd_sq;
    }
    sum  // Same as {return sum};
}

fn fifth_root(num: i32) -> i32 {
    if num == 0 || num ==1 {
        return num;
    }

    let mut root = 0;
    while root*root*root*root*root <= num{
        root += 1;
    }

    return root - 1;
}
