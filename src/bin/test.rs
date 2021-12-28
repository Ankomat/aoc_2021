fn main() {
    let a = 0b1111_1111u8;
    let b = 0b1111_1111u8;
    let f = 0b0011_0000u8;

    println!("a & f {:b}", a & f);
    println!("b & !f {:b}", b & !f);

}
