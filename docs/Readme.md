---
title: Documents
date: 
tags: ["docs", "", "", "", ""]
draft: false
summary: Some document
---

- [Introduction](#introduction)
- [From vec to strings](#from-vec-to-strings)
- [Features](#features)

## Introduction

I used enquire to allow users to pick options 

```rust
use inquire::Select;

pub fn get_build_script(build: String) {
    let os_versions = vec![
        "linux-arm64",
        "linux-armv7",
        "linux-x64",
        "macos-arm64",
        "macos-x64",
        "windows-x64",
    ];

    let answer = Select::new("What is your os version", os_versions)
        .prompt()
        .unwrap();

    println!("{}", answer)
}


```


`https://github.com/mikaelmello/inquire`



## From vec<u8> to strings


```rust
 let content = file
    .into_iter()
    .map(|v| v as char)
    .collect::<Vec<char>>()
    .into_iter()
    .collect::<String>();
 ```


## Features

- Dynamically adds output link to html if not already created
- Setups up tailwind using the latest builds from official tailwind repo
