fn main() {
    // signed integers
    let n1: i8 = -128;
    println!("n1: {}", n1);
    println!("{}", std::mem::size_of_val(&n1));

    // unsigned integers
    let n2: u8 = 255;
    println!("n2: {}", n2);
    println!("{}", std::mem::size_of_val(&n2));

    // floating
    let f1: f32 = 5.25;
    let f2 = 5.25f32;
    println!("f1: {}", f1);
    println!("f2: {}", f2);
    println!("{}", std::mem::size_of_val(&f1));

    // char
    let c1 = 'a';
    println!("c1: {}", c1);
    println!("{}", std::mem::size_of_val(&c1));

    // bool
    let b1 = true;
    println!("b1: {}", b1);
    println!("{}", std::mem::size_of_val(&b1));

    // unit type
    let u1 = ();
    println!("{}", std::mem::size_of_val(&u1));


}
