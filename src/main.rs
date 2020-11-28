use unvec::Vec;
fn main() {
    println!("Hello, world!");
    let mut vec = Vec::new();
    vec.push(5);
    vec.push(6);
    vec.push(7);

    println!("{:?}", vec.pop());
    println!("{:?}", vec.pop());
    println!("{:?}", vec.pop());
    println!("{:?}", vec.pop());

    let mut zvec = Vec::new();
    zvec.push(());

    println!("{:?}", zvec[0]);
}
