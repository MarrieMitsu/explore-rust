fn main() {
    // signed integers
    let n1: i8 = -128;
    println!("n1: {}, size: {}", n1, std::mem::size_of_val(&n1));

    // unsigned integers
    let n2: u8 = 255;
    println!("n2: {}, size: {}", n2, std::mem::size_of_val(&n2));

    // floating
    let f1: f32 = 5.25;
    let f2 = 5.25f32;
    println!("f1: {}, size: {}", f1, std::mem::size_of_val(&f1));
    println!("f2: {}, size: {}", f2, std::mem::size_of_val(&f2));

    // char
    let c1 = 'a';
    println!("c1: {}, size: {}", c1, std::mem::size_of_val(&c1));

    // string
    let sl1 = "a";
    println!("sl1: {}, size: {}", sl1, std::mem::size_of_val(&sl1));
    println!("sl1 slices: {}, size: {}", sl1, std::mem::size_of_val(&sl1[..]));

    let mut sl2 = String::from("string");
    sl2.push_str("s");
    println!("sl2: {}, size: {}", sl2, std::mem::size_of_val(&sl2));
    println!("sl2 slices: {}, size: {}", sl2, std::mem::size_of_val(&sl2[..]));

    // bool
    let b1 = true;
    println!("b1: {}, size: {}", b1, std::mem::size_of_val(&b1));

    // unit type
    let u1 = ();
    println!("u1 ,size: {}", std::mem::size_of_val(&u1));

    // tuple
    let tup =  (500, 5.6, 1);
    let (x, y, _z) = tup;
    println!("tup: {:?} size: {}", tup, std::mem::size_of_val(&tup));
    println!("x: {}, size: {}", x, std::mem::size_of_val(&x));
    println!("y: {}, size: {}", x, std::mem::size_of_val(&y));

    // array
    let a: [i8; 5] = [1, 2, 3 ,4 ,5];
    println!("a: {:?} size: {}", a, std::mem::size_of_val(&a));
    println!("{}", std::mem::size_of::<[i16; 1]>());
    println!("{}", std::mem::size_of::<[f32; 1]>());


}
