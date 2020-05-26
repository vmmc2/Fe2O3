impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        //A solucao a seguir tem complexidade espacial O(n).
        //E apresenta complexidade temporal O(1).
        
        
        //1) Se tirarmos o XOR entre 0 e um bit 'b', vamos retornar aquele bit 'b'
        //2) Se tirarmos o XOR de dois bits 'a' iguais, teremos 0 como resultado.
        //3) 'a' XOR 'a' XOR 'b' = ('a' XOR 'a') XOR 'b' = 0 XOR 'b' = 'b'
        //Basta a gente tirar o XOR de todos esses numeros presentes no array ao mesmo tempo. Isso funciona pq
        //todos eles estao em pares (Ou seja, o XOR deles vai se 'cancelar', retornando 0) e assim so vai sobrar
        //aquele numero que aparece uma unica vez (que eh o numero procurado (resposta)).
        let mut answer: i32 = 0;
        for i in (0..nums.len()){
            answer = answer ^ nums[i];
        }
        return answer;
    }
}
