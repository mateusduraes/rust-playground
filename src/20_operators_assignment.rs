/*
  https://learning-rust.github.io/docs/a9.operators.html#Assignment-and-Compound-Assignment-Operators
  The = operator is used to assign a name to a value or a function. Compound Assignment Operators are created by composing one of + - * / % & | ^ << >> operators with = operator.
*/
let mut a = 2;

a += 5; //2 + 5 = 7
a -= 2; //7 - 2 = 5
a *= 5; //5 * 5 = 25
a /= 2; //25 / 2 = 12 not 12.5 👉👉👉 It's an integer (default type is i32)
a %= 5; //12 % 5 = 2

a &= 2; //10 && 10 -> 10 -> 2
a |= 5; //010 || 101 -> 111 -> 7
a ^= 2; //111 != 010 -> 101 -> 5
a <<= 1; //'101'+'0' -> 1010 -> 10
a >>= 2; //101̶0̶ -> 10 -> 2