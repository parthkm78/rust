struct Person{
    name :  String,
    age : u32
}

fn main(){
    let person1 = Person{
        name : String::from("Prince Zeta"),
        age : 32
    };

    println!("Name : {} & Age : {}", person1.name, person1.age);
}