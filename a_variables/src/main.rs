const START_APPLICATIONS: i32 = 9;
const START_READY: i32 = 2;

fn main() {
    let (mut applications, ready) = (START_APPLICATIONS, START_READY);
    println!("{} of my {} applications are ready", ready, applications);
    applications -= ready;
    println!("{} applications left", applications);
}
