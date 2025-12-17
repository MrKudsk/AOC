
import fs from "fs/promises";

function partOne(grid: string[]) {
  let total = 0;

  const ranges = grid![0].split("\n").map(line => line.split("-").map(Number));
  // console.log(ranges);
  const numbers = grid![1].split("\n").map(Number);
  // console.log(numbers);
  for (const n of numbers) {
    if (n == 0) continue;
    for (const r of ranges) {
      if (n >= r![0] && n <= r![1]) {
        total++;
        break;
      }
    }
  }
  console.log("PartOne", total);
}

function partTwo(grid: string[]) {
  let total = 0;

  console.log("PartTwo", total);
}

// const input = await fs.readFile("./import-sample.txt", "utf-8");
const input = await fs.readFile("./import.txt", "utf-8");


partOne(input.split("\n\n"));
// partTwo(input.slice(0, -1).split("\n"));
