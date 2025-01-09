// looping triangle.
for (let n = "#"; n.length < 8; n += "#") {
  console.log(n);
}

//fizzbuzz
let fizz = "fizz";
let buzz = "buzz";

for (let count = 0; count <= 100; count++) {
  if (count % 3 === 0) {
    console.log(fizz);
  }
  if (count % 5 === 0) {
    console.log(buzz);
  }
}

//chessboard
let chessboard = [];
let times = 8;

for (let j = 0; j < times; j++) {
  let row = "";
  for (let i = 0; i < times; i++) {
    if ((j + i) % 2 === 0) {
      row += "#";
    } else {
      row += " ";
    }
  }
  chessboard.push(row);
}

console.log(chessboard.join("\n"));
