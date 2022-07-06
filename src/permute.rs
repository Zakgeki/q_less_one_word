#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gen_unique_perms() {
        let result = gen_unique_perms(["a","b","c"].to_vec());
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
pub fn gen_unique_perms<T: Clone>( mut arr: Vec<T> ) -> Vec<Vec<T>> {
    let mut perms = vec![];
    let k = arr.len();
    gen_perms(k, &mut arr, &mut perms);
    perms
}

fn gen_perms<T: Clone>(k: usize, arr: &mut Vec<T>, out: &mut Vec<Vec<T>>) {
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
