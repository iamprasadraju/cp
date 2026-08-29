// #include <bits/stdc++.h>
// a non-standard header file that automatically includes every standard library and STL header available in the GNU GCC compiler collection.


// boilerplate code (cpp Template)

#include <iostream> // standard library header for input and output streams.
#include <cstdio> // includes the C-style standard I/O library

using namespace std;

void in(); // cin input
void out(); // cout output
void cscanf(); // scanf
void cprintf(); // printf
string read_line(); // read a whole line from the input


int main(){
  read_line();
  //code
  return 0;
}

// g++ -std=c++11 -O2 -Wall main.cpp -o main
// The compiler follows the C++11 standard (-std=c++11), optimizes the code (-O2) and shows warnings about possible errors (-Wall).

// -----------------------------------------
// #include <iostream>
// #include <string>

// cin - input
void in(){
  int a, b;
  string x;
  cin >> a >> b >> x;
}

// cout - output
void out(){
  int a = 123, b = 456;
  string x = "monkey";
  cout << a << " " << b << " " << x << "\n"; // '\n' works faster than endl, because endl always causes a flash operation.
}

/* make input and output more efficient

ios::sync_with_stdio(0);
cin.tie(0);

*/


// -----------------------------------------
// C functions scanf and printf usually a bit faster.
// #include <cstdio>

void cscanf(){
  int a, b; 
  scanf("%d %d", &a, &b);
}

void cprintf(){
  int a = 123, b = 456;
  printf("%d %d\n", a, b);
}

// file read and write

void fread(){
  freopen("input.txt", "r", stdin);
  freopen("output.txt", "W", stdout);

  int x;
  cin >> x;

  cout << x * 2 << '\n';
  
}



// -----------------------------------------
// read a whole line from the input

string read_line(){
  string s;
  getline(cin, s);


  // loop to read each line
  while (cin >> x){
    // code
  }
  
  return s;
}



// -------------------------------------------
// working with numbers

/*
  10        int
  10L       long
  10LL      long long
  10U       unsigned int
  10ULL     unsigned long long
*/

void numbers(){
  long long x = 123456789123456789LL; // LL - Treat this integer literal as a long long

  // integer overflow

  int a = 123456789;
  long long b = a * a; 
  cout << b << '\n'; // output : -1757895751

  // fix

  long lon b = 1LL * a * a;
  long long b = static_cast<long long>(a) * a;
}

// Module arithmetic
