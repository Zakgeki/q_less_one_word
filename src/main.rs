// This simple program is built to generate all 12 letter
// combinations of the dice in the game q-less
//
// eventually this will be multithreaded

use std::env;
use std::fs;
use std::io::Write;

// generates permutations form a given vector
// could be of any type
//

fn gen_unique_perms<T: Clone>( mut arr: Vec<T> ) -> Vec<Vec<T>> {
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

fn main() {
    let d00 = [ "h", "r", "n" ];
    let d01 = [ "a", "e", "o" ];
    let d02 = [ "y", "b", "l", "m" ];
    let d03 = [ "h", "t", "p", "w" ];
    let d04 = [ "i", "n", "y", "o" ];
    let d05 = [ "l", "g", "d", "r" ];
    let d06 = [ "c", "m", "t", "s" ];
    let d07 = [ "c", "b", "d", "j", "t" ];
    let d08 = [ "a", "e", "i", "o", "u" ];
    let d09 = [ "d", "f", "r", "l", "w" ];
    let d10 = [ "k", "v", "f", "g", "p" ];
    let d11 = [ "s", "b", "z", "x", "n", "k" ];

    let mut file = fs::File::create("out.txt").expect("create failed");


    for i00 in 0..d00.len() {
        for i01 in 0..d01.len() {
            for i02 in 0..d02.len() {
                for i03 in 0..d03.len() {
                    for i04 in 0..d04.len() {
                        for i05 in 0..d05.len() {
                            for i06 in 0..d06.len() {
                                for i07 in 0..d07.len() {
                                    for i08 in 0..d08.len() {
                                        for i09 in 0..d09.len() {
                                            for i10 in 0..d10.len() {
                                                for i11 in 0..d11.len() {
                                                    let dice_start_perm = vec![
                                                        d00[i00], d01[i01], d02[i02], d03[i03],
                                                        d04[i04], d05[i05], d06[i06], d07[i07],
                                                        d08[i08], d09[i09], d10[i10], d11[i11]
                                                    ];
                                                    // Me trying a crate that has heaps algo already implemented
                                                    // let out = format!("{:?}", permute::permute(dice_start_perm));
                                                    let out = format!("{:?}", gen_unique_perms(dice_start_perm));
                                                    println!("{:?}", out);
                                                    file.write_all(out.as_bytes()).expect("write failed");
                                                    
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Done.");
}


