fn main() {
    /*
       i8, i16, i32, i64, i128, and isize => signed Integer
       u8, u16, u32, u64, u128 and usize => unsigned Integer

       isize, usize is different by computer architecture
       For instance, `isize` on 32 bits computer is same i32

       If type is not clarified, when integer variable is declared. By default, a variable has an i32 type
    */

    let my_number: u8 = 100;
    let my_other_number = 50; // i32

    // type inference
    /*
       By default, other integer type can't be calculated. (ex. u8 + u16)
       But, when one variable's type is not clarified, complier infer type by other variable
    */
    let third_number = my_number + my_other_number;
}
