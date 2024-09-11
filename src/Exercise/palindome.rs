fn main()
{
    let string1 = "palap";

    // check if palindrome or not
    if is_palindrome(string1)
    {
        println!("{} is palindrome", string1);
    }
    else {
        println!("{} is not palindrome", string1);
    }
     // check if palindrome or not
     if !is_palindrome1(string1)
     {
         println!("with loop : {} is palindrome", string1);
     }
     else {
         println!("with loop : {} is not palindrome", string1);
     }
}

// rev string and check
fn is_palindrome(s: &str) -> bool {
    let s_lower =  s.to_lowercase();
    s_lower.chars().eq(s_lower.chars().rev())
}
// more efficient solution
//for loop with mayching char from first and last
fn is_palindrome1(s: &str) -> bool {
    let s_lower =  s.to_lowercase();
    // count no of char in string
    let l = s_lower.chars().count();
    let mut flag: bool =  false;
    for n in 0..l{
        if s_lower.chars().nth(n).unwrap() != s_lower.chars().nth(l-n-1).unwrap(){
            flag = true
        }
    }
    return flag
}