import lcm from "npm:compute-lcm"

const [instructions, ...input] = Deno
  .readTextFileSync("./data/8.txt")
  .replace("\n", "")
  .split("\n");

const kv = input
  .map((i) => {
    const [k, v] = i.split(" = (");
    const [L, R] = v.replace(")", "").split(", ");
    return {[k]: {L , R}};
  })
  .reduce((a, b) => ({...a, ...b}), {});

const directions = instructions.split("") as unknown as ("R" | "L")[];

console.log(`Day 1, challenge 1: ${findZZZ(directions, kv)}`);
console.log(`Day 1, challenge 2: ${findxxZ(directions, kv)}`);

function findxxZ(
  directions: ("R" | "L")[],
  kv: Record<string, {R: string, L: string}>,
){
  const nextKeys = Object.keys(kv).filter(key => key.endsWith('A'));
  const iValues = [];
  for (let nextKey of nextKeys) {
    let i = 0
    while (!nextKey.endsWith("Z")) {
    nextKey = kv[nextKey][directions[i % directions.length]];
      i++;   
    }
    iValues.push(i);
  }
  return lcm(...iValues);
}

function findZZZ(
  directions: ("R" | "L")[],
  kv: Record<string, {R: string, L: string}>,
){
  let nextKey = "AAA";
  let i = 0
  while (nextKey !== "ZZZ") {
    nextKey = kv[nextKey][directions[i % directions.length]];
    i++;
  }
  return i;
}

function findZZZRecursive(
    directions: ("R" | "L")[],
    kv: Record<string, {R: string, L: string}>,
    i = 0,
    nextKey = "AAA"
  ){
  for (const direction of directions) {
    i++;
    nextKey = kv[nextKey][direction];
    if (nextKey === "ZZZ") return i;
  }
  return findZZZRecursive(directions, kv, i, nextKey);
}

Deno.bench("Loop", ()=>{findZZZ(directions.slice(), Object.assign({}, kv))});
Deno.bench("Recursion", ()=>{(findZZZRecursive)(directions.slice(), Object.assign({}, kv))});
