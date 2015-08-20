use std::env;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("Please provide a DNA sequence. Example: revcomp ATGCGATTCGA");
    } else {
        println!("{}", revcomp(args[1].trim()));
    }
}

fn revcomp(dna: &str) -> String{
    // result vector
    let mut rdna: String = String::with_capacity(dna.len()); 

    // iterate through the input &str
    for c in dna.chars().rev() {
        // test the input
        match is_dna(c) {
            false => panic!("Input sequence base is not DNA: {}", dna),
            true => rdna.push(switch_base(c))
        }
    }
    rdna
}

fn switch_base(c:char) -> char {
    match c {
        'a' => 't' ,
        'c' => 'g' ,
        't' => 'a' ,
        'g' => 'c' ,
        'u' => 'a',
        'A' => 'T' ,
        'C' => 'G' ,
        'T' => 'A' ,
        'G' => 'C',
        'U' => 'A',
        _ => 'N'
    }
}

fn is_dna(dna: char) -> bool {
    match dna {
        'A' | 'a' | 'C' | 'c' | 'G' | 'g' | 'T' | 't' | 'U'| 'u'  => true,
        _ => false
    }
}

#[test]
fn canary() {
    // nothing here, just testing the test environment
}

#[test]
fn test_is_dna() {
    assert!(is_dna('A'))
}

#[test]
#[should_panic]
fn test_is_dna_false() {
    assert!(is_dna('z'))
}

#[test]
fn test_revcomp() {
    // ATGC =>  GCAT
    assert_eq!("GCAT".to_string(), revcomp("ATGC"))
}

#[test]
#[should_panic]
fn test_revcomp_invalid_str() {
    revcomp("turtle");
}

