fn main()
{
    let mut num:i32 = 10;
    change_value(&mut num);
    println!("Midified value {}", num);
}
fn change_value(value : &mut i32){
    *value = *value + 1;
}