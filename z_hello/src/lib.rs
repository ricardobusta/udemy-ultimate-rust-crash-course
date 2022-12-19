pub fn greet(){
    println!("Greetings!");
}

pub fn if_statement(num: i32) -> &'static str{
    let msg = if num==5 {
        "five"
    } else if num == 3 {
        "three"
    } else {
        "other"
    };
    msg
}

pub fn loops(){
    let mut x = 0;
    'tick: loop { // named break
        loop{
            loop {
                x+=1;
                if x > 3 {
                    break 'tick;
                    // also works with continue 'tick;
                }
            }
        }
    }

    x=0;
    loop { // do while
        x+=1;
        println!("do while {}", x);
        if x > 3 { break; }
    }

    x=0;
    loop { // regular while
        if x > 3 { break; }
        x+=1;
        println!("while {}", x);
    }

    for num in [1,2,3].iter(){
        println!("for loop a {}", num);
    }

    for num in 1..=3{ // = makes inclusive, otherwise exclusive
        println!("for loop b {}", num);
    }

    for (x, y) in [(2,true), (5,false)].iter(){
        println!("for loop c ({}, {})", x, y);
    }
}

pub fn strawberry(){
    let s = "🍓".to_string();
    let f = || {println!("{}", s)};

    f();
}

pub fn closure_fun(){
    let mut v = vec![2, 4, 5];

    let result = v.iter()
        .map(|x| x * 3)
        .filter( |x| *x > 10)
        .fold(0, |acc, x| acc + x);

    println!("fold result {}", result);
}