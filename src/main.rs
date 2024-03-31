use std::io::{stdin, stdout, Write};
fn powmod(mut base: i32,mut exponent: i32, modulus:i32)->i32{
    let mut result:i32 = 1;
    base = base % modulus;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base)% modulus;
        }

        exponent = exponent >> 1;
        base = (base*base) % modulus;
    }
     result
}

fn order(p:i32,  b:i32)->i32
{
    if gcd(p, b) != 1 {
        println!("p and b are not co-prime.\n");
        return -1;
    }
    let mut k = 3;
    loop{
        if powmod( b,  k, p) == 1{
            return k;
        }   
        k+=1;
    }
}

fn gcd( a: i32,b: i32) -> i32 {
    if b == 0 {
        a
    }else{
        gcd(b, a % b)
    }
}

fn convertx2e(mut x: i32, _e :&mut i32) -> i32 {
    while x % 2 == 0 {
        x /= 2;
        *_e+=1;
    }
    x
}

fn stonelli(n:i32, p:i32)->i32
{
    //  a and p should be coprime for finding the modular
    // square root
    if gcd(n, p) != 1{
        println!("a and p are not coprime\n");
        return -1;
    }
    //  If below expression return (p - 1)  then modular
    // square root is not possible
    if powmod(n, (p - 1) / 2, p) == (p - 1){
        println!("no sqrt possible\n");
        return -1;
    }
    //  expressing p - 1, in terms of s * 2^e,  where s
    // is odd number
    let mut e=0;
    let s = convertx2e(p - 1, &mut e);

    //  finding smallest q such that q ^ ((p - 1) / 2)
    //  (mod p) = p - 1
    let mut q=2;
    while powmod(q, (p - 1) / 2, p) != (p - 1) {
        q += 1;
    }

    //  Initializing variable x, b and g
    let mut x = powmod(n, (s + 1) / 2, p);
    let  mut b = powmod(n, s, p);
    let  g = powmod(q, s, p);
    let mut r = e;
    // keep looping until b become 1 or m becomes 0
    loop{
        let mut m=0;
        while m < r {
            if order(p, b) == -1 {
                return -1;
            }
            if order(p, b) == powmod(2, m, p ) {
                break;
            }
            m+=1;
        }
        if m == 0 {
            return x;
        }
        x = (x * powmod(g, powmod(2, r - m - 1, p ), p)) % p;
        let temp = powmod(g, powmod(2, r - m, p ), p);
        let g = temp;
        b = (b * g) % p;

        if b == 1{
            return x;
        }
        r = m;
    }
}

fn main(){
    let p=113;
    let mut s = String::new();
    print!("Enter an integer for n: ");
    stdout().flush().expect("Failed to flush stdout");
    stdin().read_line(&mut s).expect("Failed to read line");
    let s = s.trim(); 
    let n: i32 = s.parse().expect("Please enter a valid integer");
    let x=stonelli(n,p);
    if x==-1 {
        println!("Modular square root is not exist\n");
    }else{
        println!("Modular square root of {} and {} is {} ",n, p, x);
    }
}