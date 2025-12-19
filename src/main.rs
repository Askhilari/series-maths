fn main() {
    let nth = 10;
    fibonnaci(0,1,nth); // Your will get lucas numbers if you change first two values to 2 and 1.
    tribonnaci(0, 0, 1, nth);
}


fn fibonnaci(firstno: i32, secondno: i32, n: i32) -> i32{
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

fn tribonnaci(firstno: i32, secondno: i32, thirdno: i32, n: i32) -> i32{
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