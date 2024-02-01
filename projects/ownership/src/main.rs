fn main() {
    { //scope of mutable string s
        let mut s = String::from("Hello");
        s.push_str(", world!");

        println!("{}", s);
    } //end of scope

    //comparing simple values on stack to String on the heap
    { //simple values on stack
        let x = 5;
        let y = x;

        println!("x: {}, y: {}", x, y);
    }

    {//String of unknown length at compile time, allocated to the heap
        let s1 = String::from("My string");
        let s2 = s1.clone(); //pointer is copied

        println!("s1: {} , s2: {}", s1, s2);
    }

    let s = String::from("The answer is!"); //s comes into scope
    takes_ownership(s); //s is moved into the function scope
    //s is no longer valid here...

    let num = 42; //num comes into scope
    makes_copy(num); //num is copied into the function scope
    //num is still in the scope here...

    let s1 = String::from("String s1 has been");
    let s2 = " modified!";
    let s3 = takes_and_gives_ownership_back(s1, " modified!"); //s2 comes into scope as return value
    println!("{s3}");
    println!("I can still use s2 ('{s2}'), it's on the stack!");

}

fn takes_ownership(s: String) { //s comes into scope
    println!("{}", s);
} // s moves out of scope and is dropped/freed

fn makes_copy(n:i32) {
    println!("{}", n);
}

fn takes_and_gives_ownership_back(mut s: String, s_push: &str) -> String { //s and s_push comes into scope
    s.push_str(&s_push); //s is modified
    s //s is returned
} //s is out of function scope