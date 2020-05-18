fn main(){
    let mut matrix: [[i64; 3]; 3] = [[1,2,3],[4,5,6],[7,8,9]];
    println!("{}", matrix[1][1]);
    
    for i in (0..3){
        for j in (0..3){
            println!("The current element in the matrix is: {}", matrix[i][j]);
        }
    }
}
