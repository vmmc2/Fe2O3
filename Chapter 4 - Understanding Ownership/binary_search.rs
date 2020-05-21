//Trying to do binary search in Rust...

fn binary_search(v: &mut Vec<i32>, n: usize, target: i32) -> bool{
    let mut low: usize = 0;
    let mut high: usize = n - 1;
    while low <= high{
        let mut mid: usize = low + (high - low)/2;
        if v[mid] == target{
            return true;
        }else if v[mid] < target{
            low = mid + 1;
        } else if target < v[mid]{
            high = mid - 1;
        }
    }
    return false;
}

fn main(){
    let mut v: Vec<i32> = Vec::new(); //creating a new mutable vector: in which all its values are signed integers of 32-bits
    for i in (0..10).rev(){
        v.push(i);
    }
    v.sort();
    for i in (0..10){ //lembre-se, o range em Rust, inclui o primeiro elemento mas eh exclusivo em relacao ao ultimo. Esse range vai na vdd de 0 ate 9
        println!("{}", v[i]);
    }
    println!("");
    let siz: usize = v.len();
    let answer: bool = binary_search(&mut v, siz, 10);
    println!("{}", answer);
}
