console.log(`Day 1, challenge 1: ${
  Deno
    .readTextFileSync("./data/1.txt")
    .split("\n")
    .map((l) => {
      const nl = l
        .split("")
        .map((c) => isNaN(Number(c)) ? NaN : Number(c))
        .filter((n) => !isNaN(n));
      return Number(`${nl.at(0)}${nl.at(-1)}`);
    })
    .flat()
    .reduce((a, b) => a + b, 0)
}`);

console.log(`Day 1, challenge 2: ${
  Deno
    .readTextFileSync("./data/1.txt")
    .split("\n")
    .map((l) => {
      const nl = l
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "4")
        .replace("five", "5e")
        .replace("six", "6")
        .replace("seven", "7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
        .split("")
        .map((c) => isNaN(Number(c)) ? NaN : Number(c))
        .filter((n) => !isNaN(n));
      return Number(`${nl.at(0)}${nl.at(-1)}`);
    })
    .flat()
    .reduce((a, b) => a + b, 0)
}`);
