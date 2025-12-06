
import fs from "fs/promises";


function partOne(intervals: string[]) {
  console.log(intervals);
  intervals.forEach((interval) => {
    const parts = interval.split("-")
    console.log(parts);
  });
}

function partTwo(rotations: string[]) {
}

const input = await fs.readFile("./import-sample.txt", "utf-8");
// const input = await fs.readFile("./import.txt", "utf-8");

partOne(input.slice(0, -1).split(","));
// partTwo(input.split("\n"));
