// This simple program is built to generate all 12 letter
// combinations of the dice in the game q-less
//
// eventually this will be multithreaded

// for file writing implementation
// use std::fs;
// use std::io::Write;

// commented out to avoid a warning
// mod permute;
// use permute::permute;

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

    let mut out = vec![];
    
    for r00 in d00 {
        for r01 in d01 {
            for r02 in d02 {
                for r03 in d03 {
                    for r04 in d04 {
                        for r05 in d05 {
                            for r06 in d06 {
                                for r07 in d07 {
                                    for r08 in d08 {
                                        for r09 in d09 {
                                            for r10 in d10 {
                                                for r11 in d11 {
                                                    let line = format!(
                                                        "{}{}{}{}{}{}{}{}{}{}{}{}",
                                                        r00, r01, r02, r03,
                                                        r04, r05, r06, r07,
                                                        r08, r09, r10, r11
                                                    );

                                                    out.push(line);
                                                    
                                                    // for when I do permutations
                                                    // let dice_start_perm = vec![
                                                    //     r00, r01, r02, r03,                                                         r00, r01, r02, r03,
                                                    //     r04, r05, r06, r07,
                                                    //     r08, r09, r10, r11
                                                    // ];
                                                    //
                                                    // let temp = format!("{:?}", permute(dice_start_perm));
                                                    // println!("{:?}", out);
                                                    //
                                                    // out.push(temp);
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
    
    // writing to file instead of printing to a string
    // weirdly enough, it's much faster to just redirect
    // in linux
    //
    // let mut file = fs::File::create("out.txt").expect("create failed");
    // for line in out {
    //     _ = writeln!(file,"{}", line);
    // }
    // println!("Done.");

    for line in out {
        println!("{}", line);
    }
}


