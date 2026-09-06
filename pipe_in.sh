#!/bin/bash


pbpaste > in.txt
# g++ -O2 -Wall atcoder.cpp && cat in.txt | ./a.out

cat in.txt | python3 cp.py
