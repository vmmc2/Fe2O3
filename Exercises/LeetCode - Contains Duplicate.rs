impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        //approach 1: Sort the array, Do not use extra space, iterate through the Vector, Check adjacent elements
        //Time Complexity: O(n*log(n))
        let mut v = nums.clone(); //Clona os dados do vetor (dados da heap)
        v.sort();
        for i in (1..v.len()){
            if v[i] == v[i - 1]{
                return true;
            }
        }
        return false;
    }
}
