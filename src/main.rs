// This simple program is built to generate all 12 letter
// combinations of the dice in the game q-less
//
// eventually this will be multithreaded


 
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use regex::Regex;

// commented out to avoid a warning
// mod permute;
// use permute::permute;

use multimap::MultiMap;

fn string_hash(s: String) -> i128  {
    let mut product = 1;

    for c in s.chars() {
        product *= char_hash(c);
    }

    product
}

fn char_hash( c: char ) -> i128 {
    match c {
        'b' =>  3,
        'c' =>  5,
        'd' =>  7,
        'a' =>  2,
        'e' => 11,
        'f' => 13,
        'g' => 17,
        'h' => 19,
        'i' => 23,
        'j' => 29,
        'k' => 31,
        'l' => 37,
        'm' => 41,
        'n' => 43,
        'o' => 47,
        'p' => 53,
        'r' => 59,
        's' => 61,
        't' => 67,
        'u' => 71,
        'v' => 73,
        'w' => 79,
        'x' => 83,
        'y' => 89,
        'z' => 97,
         _  =>  1
    }
}

fn main() {
    // dice to roll (duplicates on each die have been removed)
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

    // load in the text file to go through
    let file = File::open("words2ElectricBoogaloo.txt").unwrap();
    let mut file_buffer = BufReader::new(file);
    let mut contents = String::new();
    file_buffer.read_to_string(&mut contents).expect("Unable to read string");
    
    // filter out any for strings of vowels since the dice only allow 3 at a time
    // (significantly narrows the search space)
    let vowel_filt = Regex::new(r"[aoeui].*[aoeui].*[aoeui].*[aoeui].*").unwrap();
    let filtered_contents = contents.lines()
                                    .filter(|line| !vowel_filt.is_match(line));
    
    // create the hashmap for the filtered file contents
    let mut data_map = MultiMap::new();

    for line in filtered_contents {
        data_map.insert(string_hash(line.trim().to_string()), line.to_string() );
    }

    // "roll" each die and calculate its hash
    let mut out: Vec<&String> = vec![];

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

                                                    // check if the rolled hash is in the map and check if it's not in 
                                                    // the output vector. If true, then add it to the output vector
                                                    if data_map.contains_key(&hash11) && !out.contains(&data_map.get(&hash11).unwrap()) {
                                                        
                                                        let entry = data_map.get_vec(&hash11).unwrap(); 
                                                        for word in entry {
                                                            out.push(word);
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

    // sort and print the results
    out.sort();

    for word in out {
        print!("{}\n", word);
    }
}


