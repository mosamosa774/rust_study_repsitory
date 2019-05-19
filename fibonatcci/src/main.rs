fn fibonatcci(n: usize, x: &mut Vec<u128>) -> &mut Vec<u128>{
    if x.len() > n {
        return x;
    }
    let x1 = fibonatcci(n-1,x)[n-1] as u128;
    let x2 = fibonatcci(n-2,x)[n-2] as u128;
    let res = x1+x2;
    ///let res = x1.wrapping_add(x2);
    (*x).push(res);
    return x;
}

fn main() {
    let mut count = 0;
    while count < 1000 {
        let n = count;
        let mut v: Vec<u128> = Vec::with_capacity(n+1);
        if n >= 0 {
            v.push(1);
        }
        if n >= 1 {
            v.push(1);
        }
        let res = fibonatcci(n,&mut v);
        println!("n:{} value:{}",n,res[n]);
        count+=1;
    }
}
