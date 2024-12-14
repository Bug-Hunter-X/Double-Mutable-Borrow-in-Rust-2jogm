fn main() {
    let mut x = 5;
    { // Limiting the scope of y
        let y = &mut x;
        *y += 1;
    }
    { // Limiting the scope of z
        let z = &mut x;
        *z +=1; 
    }
    println!("x = {}", x);
}