---
layout: post
title:  "DNA reverse complement"
date:  2015-08-20 14:55:57 -0400
categories:
- blog
- rust
- bioinformatics
permalink: dna-reverse-complement
description: A small Rust program to provide the reverse complement of a DNA sequence

---

In the [last post]({{ "/getting-started" | prepend: site.baseurl }}), we outlined to the requirements for a command line program to reverse complement a DNA sequence: 

- The program must take in an argument consisting of DNA sequence via STDIN 
- If the input is not DNA (consists only of "ATGC" characters), then throw an error
- Else output the reverese complement sequence as STDOUT


Let's hop to it! We'll start with writing the tests for a small function to decide whether any given character is a DNA character in `main.rs`. While we could check the entire input with soem regular expression or pattern, here we will do the simple thing and check each DNA base as we iterate through each character of the input. 

```rust
#[test]
fn canary() {
    // testing the Rust environment
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

```

Next let's take a stab at the function itself. 

```rust 
fn main() {
    // put code here
}

fn is_dna(dna: char) -> bool {
    match dna {
        'A' | 'a' | 'C' | 'c' | 'G' | 'g' | 'T' | 't' | 'U'| 'u'  => true,
        _ => false
    }
}

```

Finally, let's run these tests and see if we have a successful set of tests and a function

```bash 

$ cargo test
   Compiling revcomp v0.1.0 (file:///..../revcomp)
     Running target/debug/revcomp-c88ffaf7524d2393

running 3 tests
test canary ... ok
test test_is_dna ... ok
test test_is_dna_false ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured

```

Now that we know the system is working and we have some minimal tests and function, let's move onto the meat of the program. The next function we shall create is the  reverse complement function `revcomp()`, but first let's write the tests

```rust 

#[test]
fn test_revcomp() {
    // ATGC =>  GCAT
    assert_eq!("GCAT", revcomp("ATGC"))
    assert_eq!("atgc", revcomp("atgc"))
    assert_eq!("GCat", revcomp("ATgc"))
}

#[test]
#[should_panic]
fn test_revcomp_invalid_str() {
    revcomp("TaGyouareit")
}

```

From the tests, we see that the revcomp function should return the sequence at each position to maintain the input case of the DNA base. Th fucntion will need to create a placeholder for the result, then iterate through the sequence in reverse to the reverse complement:

```rust

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

```

Notice that I decided the program should segfault with a standard Rust error should it be provided with a bad input sequence. We could have handled the error better, but this will suffice for now. 

As a final stage, we use the `main` function to parse the first command line argument and provide the reverse complement, given that it is a valid DNA sequence.

```rust

use std::env;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("Please provide a DNA sequence. Example: revcomp ATGCGATTCGA");
    } else {
        println!("{}", revcomp(args[1].trim()));
    }
}

```

With all the pieces in place, let's run the test suite. 

```bash

$ cargo test
   Compiling revcomp v0.1.0 (file:///.../revcomp)
src/main.rs:3:1: 10:2 warning: function is never used: `main`, #[warn(dead_code)] on by default
src/main.rs:3 fn main() {
src/main.rs:4     let args = env::args().collect::<Vec<_>>();
src/main.rs:5     if args.len() < 2 {
src/main.rs:6         println!("Please provide a DNA sequence. Example: revcomp ATGCGATTCGA");
src/main.rs:7     } else {
src/main.rs:8         println!("{}", revcomp(args[1].trim()));
              ...
     Running target/debug/revcomp-c88ffaf7524d2393

running 5 tests
test canary ... ok
test test_is_dna ... ok
test test_is_dna_false ... ok
test test_revcomp ... ok
test test_revcomp_invalid_str ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured

```

As a final step we run the program itself to see the outputs

```bash 
$./target/debug/revcomp ATGCGATTCggatacaAATTAG
CTAATTtgtatccGAATCGCAT
$ ./target/debug/revcomp NotaDNAsequence
thread '<main>' panicked at 'Input sequence base is not DNA: NotaDNAsequence', src/main.rs:23
```

Success! You can view the full source tree for this program at [sources/rust/revcomp](https://github.com/delagoya/rusty-bio/tree/gh-pages/sources/rust/revcomp)
