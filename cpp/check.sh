#!/bin/bash


pbpaste > in.txt

g++ -O2 -Wall main.cpp && cat in.txt | ./a.out

rm a.out

> in.txt
