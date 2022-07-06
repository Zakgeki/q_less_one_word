#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gen_unique_perms() {
        let result = permute(["a","b","c"].to_vec());
        let desired = vec![ 
            vec!["a","b","c"], 
            vec!["b","a","c"], 
            vec!["c","a","b"], 
            vec!["a","c","b"], 
            vec!["b","c","a"], 
            vec!["c","b","a"]
        ];

        assert_eq!(result, desired);
    }
}

/// generates permutations form a given vector
pub fn permute<T: Clone>( mut arr: Vec<T> ) -> Vec<Vec<T>> {
    fn gen_perms<N: Clone>(k: usize, arr: &mut Vec<N>, out: &mut Vec<Vec<N>>) {
        if k == 0 {
            out.push( arr.clone() );
        } else {
            gen_perms( k - 1, arr , out);
            
            for i in 0..k - 1 {
                if k % 2 == 0 {
                    arr.swap(i, k - 1);
                } else {
                    arr.swap(0, k - 1 );
                }
                
                gen_perms( k - 1, arr, out );
            }
        }
    }
    
    let mut perms = vec![];
    let k = arr.len();
    gen_perms(k, &mut arr, &mut perms);
    // for perm in perms {
    //     println!("{:?}", perm);
    // }
    // println!("{:?}", perms);
    perms
}
