import { Calculator } from "https://bitbucket.org/iccee0/calc/raw/89461539e71768c494647c089f6eb1130e8c1172/calc.ts";

// Because speed is not an issue, I'm using a calculator class to do the math without having to write it myself. fkkk yeaaa
const calc = new Calculator();

const histories = Deno
  .readTextFileSync("./data/9.txt")
  .split("\n")
  .map((line) => line.split((" ")).map((num) => +num));

let sum = 0;
let sum2 = 0;
for (const history of histories) {
  const lastdivisors = [history.at(-1)];
  const firstdivisors = [history.at(0)];
  let currentHistory = history;
  
  while (!currentHistory.every((num) => num === 0)) {
    currentHistory = currentHistory.reduce((acc, curr, idx, src) => {
        if (idx < src.length - 1) acc.push((src[idx + 1] || 0) - curr);
        return acc;
    }, [] as number[])    
    lastdivisors.push(currentHistory.at(-1));
    firstdivisors.push(currentHistory.at(0));        
  }
  sum += lastdivisors.reduce((acc, curr) => calc.plus((acc||0), (curr||0)), 0) || sum;
  sum2 += firstdivisors.reverse().reduce((acc, curr) => calc.minus((curr||0), (acc||0)), 0) || 0;
}

console.log(`Day 9, challenge 1: ${sum}`);
console.log(`Day 9, challenge 2: ${sum2}`);


