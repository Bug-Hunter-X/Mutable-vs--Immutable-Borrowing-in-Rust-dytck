fn main() {
    let mut x = 5;
    let y = &mut x;
    *y = 10;
    let z = &x; // This is fine
    println!("x = {}", x);
    let mut v = vec![1, 2, 3];
    {
        let u = &v; //&v is immutable, so this is ok
        for i in u {
            println!("Value: {}", i);
        }
    }
    v.push(4); //Correct, this is mutable
    println!("Vector v: {:?}", v);
}