fn main() {
    let s = String::from("Hello, world!");
    let (s2, s2_size) = no_reference_str_len(s); //ownership of s transferred to function
    //s is now out of scope and is dropped, so we use s2

    println!("{}", s2); //we have to use s2, s1 is dropped after function call
    println!("Has length: {}", s2_size);

    let s3 = String::from("Hello, universe!");
    let s3_size = ref_str_len(&s3); //we only pass the ref, so borrow it to the function
    //s3 is still owned by this scope
    println!("{}", s3);
    println!("Has length: {}",s3_size);

    let mut s4 = String::from("Hello, "); //mutable string
    let audience1 = "all worlds!";
    let audience2 = " And all universes!";

    change(&mut s4, audience1); //we can change s4, because the ref is mutable
    println!("{}", s4);
    change(&mut s4, audience2); //we can change s4 again
    println!("{}", s4);
}

fn no_reference_str_len(s:String) -> (String, usize) { //we take ownership of s
    let size = s.len();
    (s, size) //we have to return s again to use it in the outer scope
}

fn ref_str_len(s:&String) -> usize { //we only borrow s by reference, not taking ownership
    s.len() //we return the string length
}

fn change(s:&mut String, push_s:&str) { //we borrow s, but we are allowed to modify it
    s.push_str(push_s);
}