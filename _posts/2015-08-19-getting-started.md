---
layout: post
title:  "Getting Started"
date:  2015-08-19 11:30:38
categories:
- blog
- rust
permalink: getting-started
description: Intro to Rust tooling, and creating a simple CLI for DNA reverse complement.
---

Other than [a good text editor](http://www.sublimetext.com/) (please don't email me about [some](https://macromates.com/) [other](https://www.gnu.org/software/emacs/) [editor](http://www.vim.org/)) and *nix terminal, you will need a few items, not the least of which is [Rust](https://www.rust-lang.org/). Since I am on a Mac, I use the excellent [Homebrew](http://brew.sh/) to install Rust: 

```bash

    brew install rust

```


Rust includes the [Cargo tool](https://doc.rust-lang.org/stable/book/hello-cargo.html) that helps out with creating the proper package layout, and helps in testing and compiling code. Let's use it to create a simple command line that will take a DNA sequence as input and give the reverse complement.

```bash

    cargo new revcomp --bin

```


This creates the following structure:

```bash

$ tree  -p  revcomp
revcomp
├── [-rw-r--r--]  Cargo.toml
└── [drwxr-xr-x]  src
    └── [-rw-r--r--]  main.rs

```

I won't go repeating the excellent content of the [Rust book](https://doc.rust-lang.org/stable/book/README.html) but you should read the sections on [Cargo](https://doc.rust-lang.org/stable/book/hello-cargo.html), the [Guessing Game](https://doc.rust-lang.org/stable/book/hello-cargo.html) and [testing](https://doc.rust-lang.org/stable/book/testing.html) in Rust. Here we will use the more simple testing pattern. 

## Requirements

- [ ] The program must take in an argument consisting of DNA sequence via STDIN 
- [ ] If the input is not DNA (consists only of "ATGC" characters), then throw an error
- [ ] Else output the reverese complement sequence as STDOUT

In the next post I will go through this program in detail. 