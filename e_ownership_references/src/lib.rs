pub fn inspect(s: &String){
    if s.ends_with("s"){
        println!("plural")
    }else{
        println!("singular")
    }
}

pub fn change(s: &mut String){
    if !s.ends_with("s") {
        s.push_str("s");
    }
}

pub fn eat(s: String) -> bool{
    s.contains("a") && s.starts_with("b")
}

pub fn bedazzle(m: &mut String){
    *m = "sparkly".to_string();
}