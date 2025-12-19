
import fs from "fs/promises";


function partOne(grid: string[]) {
  let total = 0;
  console.log("PartOne", total);
}

function partTwo(grid: string[]) {
  let total = 0;

  console.log("PartTwo", total);
}

const input = await fs.readFile("./import-sample.txt", "utf-8");
// const input = await fs.readFile("./import.txt", "utf-8");

partOne(input.slice(0, -1).split("\n"));
// partTwo(input.slice(0, -1).split("\n"));
