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

use num::bigint::BigUint;
// use multimap::MultiMap;

fn string_hash(s: String) -> BigUint {
    let mut product = BigUint::from( 1 as usize );
    println!("{}",s);
    for c in s.chars() {
        product *= char_hash(c);
        // println!("{}", char_hash(c));
    }

    product
}

fn char_hash( c: char ) -> BigUint {

    match c {
        'a' => { println!("{}",  2 ); BigUint::from(  2 as usize )},
        'b' => { println!("{}",  3 ); BigUint::from(  3 as usize )},
        'c' => { println!("{}",  5 ); BigUint::from(  5 as usize )},
        'd' => { println!("{}",  7 ); BigUint::from(  7 as usize )},
        'e' => { println!("{}", 11 ); BigUint::from( 11 as usize )},
        'f' => { println!("{}", 13 ); BigUint::from( 13 as usize )},
        'g' => { println!("{}", 17 ); BigUint::from( 17 as usize )},
        'h' => { println!("{}", 19 ); BigUint::from( 19 as usize )},
        'i' => { println!("{}", 23 ); BigUint::from( 23 as usize )},
        'j' => { println!("{}", 29 ); BigUint::from( 29 as usize )},
        'k' => { println!("{}", 31 ); BigUint::from( 31 as usize )},
        'l' => { println!("{}", 37 ); BigUint::from( 37 as usize )},
        'm' => { println!("{}", 41 ); BigUint::from( 41 as usize )},
        'n' => { println!("{}", 43 ); BigUint::from( 43 as usize )},
        'o' => { println!("{}", 47 ); BigUint::from( 47 as usize )},
        'p' => { println!("{}", 53 ); BigUint::from( 53 as usize )},
        'r' => { println!("{}", 59 ); BigUint::from( 59 as usize )},
        's' => { println!("{}", 61 ); BigUint::from( 61 as usize )},
        't' => { println!("{}", 67 ); BigUint::from( 67 as usize )},
        'u' => { println!("{}", 71 ); BigUint::from( 71 as usize )},
        'v' => { println!("{}", 73 ); BigUint::from( 73 as usize )},
        'w' => { println!("{}", 79 ); BigUint::from( 79 as usize )},
        'x' => { println!("{}", 83 ); BigUint::from( 83 as usize )},
        'y' => { println!("{}", 89 ); BigUint::from( 89 as usize )},
        'z' => { println!("{}", 97 ); BigUint::from( 97 as usize )},
         _  => BigUint::from(  0 as usize )
    }
}

fn main() {
    const D00: [ char; 3 ] = [ 'h', 'r', 'n' ];
    const D01: [ char; 3 ] = [ 'a', 'e', 'o' ];
    const D02: [ char; 4 ] = [ 'y', 'b', 'l', 'm' ];
    const D03: [ char; 4 ] = [ 'h', 't', 'p', 'w' ];
    const D04: [ char; 4 ] = [ 'i', 'n', 'y', 'o' ];
    const D05: [ char; 4 ] = [ 'l', 'g', 'd', 'r' ];
    const D06: [ char; 4 ] = [ 'c', 'm', 't', 's' ];
    const D07: [ char; 5 ] = [ 'c', 'b', 'd', 'j', 't' ];
    const D08: [ char; 5 ] = [ 'a', 'e', 'i', 'o', 'u' ];
    const D09: [ char; 5 ] = [ 'd', 'f', 'r', 'l', 'w' ];
    const D10: [ char; 5 ] = [ 'k', 'v', 'f', 'g', 'p' ];
    const D11: [ char; 6 ] = [ 's', 'b', 'z', 'x', 'n', 'k' ];

    // let mut out = vec![];

    for r00 in D00 {
        let hash00 = char_hash( r00 );
       
        for r01 in D01 {
            let hash01 = hash00.clone() * char_hash( r01 );

            for r02 in D02 {
                let hash02 = hash01.clone() * char_hash( r02 );

                for r03 in D03 {
                    let hash03 = hash02.clone() * char_hash( r03 );

                    for r04 in D04 {
                        let hash04 = hash03.clone() * char_hash( r04 );

                        for r05 in D05 {
                            let hash05 = hash04.clone() * char_hash( r05 );

                            for r06 in D06 {
                                let hash06 = hash05.clone() * char_hash( r06 );

                                for r07 in D07 {
                                    let hash07 = hash06.clone() * char_hash( r07 );

                                    for r08 in D08 {
                                        let hash08 = hash07.clone() * char_hash( r08 );

                                        for r09 in D09 {
                                            let hash09 = hash08.clone() * char_hash( r09 );

                                            for r10 in D10 {
                                                let hash10 = hash09.clone() * char_hash( r10 );

                                                for r11 in D11 {
                                                    let hash11 = hash10.clone() * char_hash( r11 );
                                                    println!("{}", hash11); 
                                                    let hash = string_hash(format!(
                                                        "{}{}{}{}{}{}{}{}{}{}{}{}",
                                                        r00, r01, r02, r03,                                                         r00, r01, r02, r03,
                                                        r04, r05, r06, r07,
                                                        r08, r09, r10, r11
                                                    ));
                                                    println!("{}", hash);
                                                    
                                                    assert_eq!(hash11, hash);
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

    // for line in out {
    //     println!("{}", line);
    // }
}


