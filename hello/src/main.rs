use hello::greet;

fn main() {
    println!("Hello, world!");

    // Variables

    const X : i32 = 100; // const
    let y = 40; // immutable
    {
        let mut z: i32 = 300; // muttable
        println!("values: x={}, y={}, z={}", X, y, z);
        z = 20;
        println!("values: x={}, y={}, z={}", X, y, z);
    }

    let shadow = 30;
    {
        let shadow = 15;
        println!("shadow={}", shadow);
    }
    println!("shadow={}", shadow);
    let mut shadow = 9;
    println!("same scope shadow={}", shadow);

    greet();
}
