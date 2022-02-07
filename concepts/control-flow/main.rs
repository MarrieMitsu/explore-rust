fn main() {

    // if expressions
    let num: u8 = 5;

    if num < 10 {
        println!("less than 10");
    } else if num > 10 {
        println!("greater than 10");
    } else {
        println!("10");
    }

    let result = if num > 5 {
        num
    } else {
        0
    };
    println!("{}", result);


    // loop expressions
    let mut count = 0;
    loop {
        println!("loop: {}", count);

        if count > 5 {
            break;
        }
        
        count += 1;
    }

    count = 0;
    'upper_loop: loop {
        println!("upper_loop: {}", count);

        let mut inner_count = 10;

        'inner_loop: loop {
            println!("inner_loop: {}", inner_count);

            if inner_count < 8 {
                break 'inner_loop;
            }
            if count == 3 {
                break 'upper_loop;
            }

            inner_count -= 1;
        }

        count += 1;
    }
    println!("{}", count);

    count = 0;
    let loop_result = loop {
        count += 1;

        if count == 5 {
            break count * 2;
        }
    };
    println!("loop_result: {}", loop_result);

    // while loop
    let arr: [u8; 5] = [1, 2, 3, 4, 5];
    let mut index = 0;

    while index < 5 {
        println!("while_loop: {}", arr[index]);

        index += 1;
    }

    // for loop
    for element in arr {
        println!("for_loop: {}", element);
    }

    for number in 0..7 {
        println!("number: {}", number);
    }

    // match expressions
    let condition: bool = false;
    match condition {
        true => println!("The condition is true"),
        false => println!("The condition is false")
    }

    let match_number: u8 = 2;
    let match_result = match match_number {
        1 => {
            println!("match_number: {}", 1);
            1 * 10
        },
        2 => {
            println!("match_number: {}", 2);
            2 * 10
        },
        _ => 0
    };
    println!("match_result: {}", match_result);

    let ref1 = &4; 
    match *ref1 {
        val => println!("{:?}", val)
    }
    println!("{:?}", ref1);

}
