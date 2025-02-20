fn main() {
    let mut x = 5;
    let y = &mut x;
    *y = 10;
    let z = &x; // This is fine
    println!("x = {}", x);
    let v = vec![1, 2, 3];
    let u = &v; //&v is immutable, so this is ok
    for i in u {
        println!("Value: {}", i);
    }
    let w = &mut v;
    w.push(4); //This is incorrect as push() is mutable
    println!("Vector w: {:?}", w);
}