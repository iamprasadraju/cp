#!/bin/bash

pbpaste > in.txt


rustc main.rs
cat in.txt | ./main

rm main
> in.txt
