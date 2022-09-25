// https://www.youtube.com/watch?v=dRunrgbBNKQ
fn main() {
    /*
       stack, heap, pointer
    */
    let my_number = 15;
    let single_reference = &my_number;
    let double_reference = &single_reference;
    let five_references = &&&&&my_number;
}
