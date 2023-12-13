import { Calculator } from "https://bitbucket.org/iccee0/calc/raw/89461539e71768c494647c089f6eb1130e8c1172/calc.ts";
import { assertEquals } from "https://deno.land/std@0.208.0/assert/mod.ts";

// Because speed is not an issue, I'm using a calculator class to do the math without having to write it myself. fkkk yeaaa
const calc = new Calculator();

function day9(input: string): { 1: number, 2: number } {
  const histories = input
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
  return { 1: sum, 2: sum2 }
}


const res = day9(Deno.readTextFileSync("./data/9.txt"));

console.log(`Day 9, challenge 1: ${res[1]}`);
console.log(`Day 9, challenge 2: ${res[2]}`);


Deno.test({name:"Day 9", fn: async t => {
  const testInput = `0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45`;

  await t.step("Challenge 1", () => {
    const res = day9(testInput);
    assertEquals(res[1], 114);
  });

  await t.step("Challenge 2", () => {
    const res = day9(testInput);
    assertEquals(res[2], 2);
  });
}});