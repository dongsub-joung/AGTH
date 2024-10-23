#[no_mangle]
pub extern "C" fn age(cy: i32, yob: i32) -> i32{
   cy - yob
}
