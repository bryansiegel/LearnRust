fn main() {
    // let x = 5;
    // let x = x + 1;
    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }
    //
    // println!("The value of x is: {x}");
    //
    // let f: bool = false;
    // let t = true;
    // println!("{} {t}", f);

    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    //
    // println!("{:?}", tup);

let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("{one}");
    println!("{five_hundred}");
    println!("{six_point_four}");

}