#!/bin/bash


pbpaste > in.txt

g++ -O2 -Wall atcoder.cpp && cat in.txt | ./a.out
