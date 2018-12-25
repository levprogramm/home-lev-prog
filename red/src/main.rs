#[macro_use] extern crate text_io ;
fn main (){
let i: i32  = read!();
let s: char  = read!();
let z: i32  = read!();
 
let zis = match s {
    '+' =>i+z,
    '-' =>i-z,
    '*' =>i * z,
    '/' =>i/z,
    '!' => fac_recurs(i),
    '^' => power(i,z),
    _ => 3,
 
    };
println!("{}", zis);

}

fn fac_recurs(n:i32) -> i32 {

    if n <= 1 {1} else {n * fac_recurs(n-1)}

}
fn power (mut t: i32 ,mut k: i32) -> i32 {
    if k == 0 {
        return 1;
    } else if k % 2 == 1 {
        return power(t, k-1) * t;
    } else {
        let b = power(t, k/2);
        return b * b;
    }
}
fn coren(q:i32 , w: i32)-> i32{
    if q * 100000 - w * w <=0{
        println!("{}", w/100);
    } else { coren(q , w+1)}
        

}
