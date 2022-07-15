// This simple program is built to generate all 12 letter
// combinations of the dice in the game q-less
//
// eventually this will be multithreaded

use std::collections::HashMap;
// for file writing implementation
use std::fs::File;
use std::io::BufReader;
use std::io::{Write, Read};

// commented out to avoid a warning
// mod permute;
// use permute::permute;

use num::bigint::BigUint;
use multimap::MultiMap;

fn string_hash(s: String) -> BigUint {
    let mut product = BigUint::from( 1 as usize );

    for c in s.chars() {
        product *= char_hash(c);
    }

    product
}

fn char_hash( c: char ) -> BigUint {

    match c {
        'a' => BigUint::from(  2 as usize ),
        'b' => BigUint::from(  3 as usize ),
        'c' => BigUint::from(  5 as usize ),
        'd' => BigUint::from(  7 as usize ),
        'e' => BigUint::from( 11 as usize ),
        'f' => BigUint::from( 13 as usize ),
        'g' => BigUint::from( 17 as usize ),
        'h' => BigUint::from( 19 as usize ),
        'i' => BigUint::from( 23 as usize ),
        'j' => BigUint::from( 29 as usize ),
        'k' => BigUint::from( 31 as usize ),
        'l' => BigUint::from( 37 as usize ),
        'm' => BigUint::from( 41 as usize ),
        'n' => BigUint::from( 43 as usize ),
        'o' => BigUint::from( 47 as usize ),
        'p' => BigUint::from( 53 as usize ),
        'r' => BigUint::from( 59 as usize ),
        's' => BigUint::from( 61 as usize ),
        't' => BigUint::from( 67 as usize ),
        'u' => BigUint::from( 71 as usize ),
        'v' => BigUint::from( 73 as usize ),
        'w' => BigUint::from( 79 as usize ),
        'x' => BigUint::from( 83 as usize ),
        'y' => BigUint::from( 89 as usize ),
        'z' => BigUint::from( 97 as usize ),
         _  => BigUint::from(  1 as usize )
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

    let file = File::open("words2ElectricBoogaloo.txt").unwrap();
    let mut file_buffer = BufReader::new(file);
    let mut contents = String::new();
    file_buffer.read_to_string(&mut contents).expect("Unable to read string");
    
    // let mut data = vec![];

    let mut data_map = MultiMap::new();

    for line in contents.split("\n") {
        data_map.insert(string_hash(line.to_string()), line.to_string() );
        // println!("hash: {}", string_hash((line.to_string())));
    }

    // for (key, values) in data_map.iter_all() {
    //     for line in values {
    //         assert_eq!(&string_hash(line.to_string()), key);
    //     }
    // }

    // for line in data {
    //     println!("{}", line);
    // }


    let mut out = HashMap::new();
    for r00 in D00 {
        let hash00 = char_hash( r00 );
       
        for r01 in D01 {
            let hash01 = &hash00 * char_hash( r01 );

            for r02 in D02 {
                let hash02 = &hash01 * char_hash( r02 );

                for r03 in D03 {
                    let hash03 = &hash02 * char_hash( r03 );

                    for r04 in D04 {
                        let hash04 = &hash03 * char_hash( r04 );

                        for r05 in D05 {
                            let hash05 = &hash04 * char_hash( r05 );

                            for r06 in D06 {
                                let hash06 = &hash05 * char_hash( r06 );

                                for r07 in D07 {
                                    let hash07 = &hash06 * char_hash( r07 );

                                    for r08 in D08 {
                                        let hash08 = &hash07 * char_hash( r08 );

                                        for r09 in D09 {
                                            let hash09 = &hash08 * char_hash( r09 );

                                            for r10 in D10 {
                                                let hash10 = &hash09 * char_hash( r10 );

                                                for r11 in D11 {
                                                    let hash11 = &hash10 * char_hash( r11 );
                                                    if data_map.contains_key(&hash11) {
                                                        
                                                       let temp = data_map.get_vec(&hash11); 
                                                       for vector in temp {
                                                            for line in vector {

                                                                out.insert(line, line);
                                                            }
                                                       }
                                                    }

                                                    
                                                    // let temp = format!( "{}{}{}{}{}{}{}{}{}{}{}{}",
                                                    //     r00, r01, r02, r03,
                                                    //     r04, r05, r06, r07,
                                                    //     r08, r09, r10, r11
                                                    // );
                                                    // let hash = string_hash(temp);
                                                    
                                                    // assert_eq!(hash11, hash);
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

    let mut final_list = vec![];
   for (_, val) in out.iter() {
        final_list.push( val );
   } 

   final_list.sort();

   for line in final_list {
        println!("{}", line);
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


