impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut answer: i32 = 1; //vai guardar quantos valores diferentes tenho...
        let mut cursor: usize = 0;
        let mut receiver: usize = 0;
        if nums.len() == 0{
            return 0;
        }
        if nums.len() == 1{
            return 1;
        }
        while cursor < nums.len(){
            if nums[receiver] != nums[cursor]{
                receiver += 1;
                nums[receiver] = nums[cursor];
                answer += 1;
            }
            cursor += 1;
        }
        return answer;
    }
}
