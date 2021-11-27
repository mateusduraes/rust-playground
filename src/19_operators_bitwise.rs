/*
  let g = a >> b; //0  (Remove b number of bits from the end of a -> o̶1̶ -> 0)
  & | ^ << >>
*/

let a = 1;
let b = 2;

let c = a & b;  //0  (01 && 10 -> 00)
let d = a | b;  //3  (01 || 10 -> 11)
let e = a ^ b;  //3  (01 != 10 -> 11)
let f = a << b; //4  (Add b number of 0s to the end of a -> '01'+'00' -> 100)
let g = a >> b; //0  (Remove b number of bits from the end of a -> o̶1̶ -> 0)