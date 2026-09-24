#!/bin/bash


pbpaste > in.txt

gcc main.c && cat in.txt | ./a.out

rm a.out

> in.txt
