#[derive(Debug)]
struct ColorRegularStruct {
    // TODO: Add the fields that the test `regular_structs` expects.
    // What types should the fields have? What are the minimum and maximum values for RGB colors?
    red: u8,
    blue: u8,
    green: u8,
}
pub fn structs_one() {
    let color_one = ColorRegularStruct {
        red: 0,
        green: 255,
        blue: 0,
    };

    println!("{:#?}", color_one);
    println!("{}", color_one.green);
    println!("{}", color_one.blue);
    println!("{}", color_one.red)
}
