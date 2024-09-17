fn main()
{
    let  s1 = "eeee";
    let s2 = "bcd";
    let smallest = find_smallest(&s1,&s2);
    println!("Smallest string is  {}", smallest);
}

fn find_smallest<'a>(v1: &'a str, v2:&'a str) -> &'a str {
    if v1<v2{
        v1
    }else 
    {
        v2
    }
}