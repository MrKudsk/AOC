
import fs from "fs/promises";

function isInvalid(num: number) {
  const stringValue = num.toString();
  return /^(\d+)\1+$/.test(stringValue);
  // const left = stringValue.slice(0, stringValue.length / 2);
  // const right = stringValue.slice(stringValue.length / 2);
  // return left === right;
}

function partOne(intervals: string[]) {
  console.log(intervals);
  let invalidIDs: number[] = [];
  intervals.forEach((interval) => {
    const [start, end] = interval.split("-").map(Number)
    for (let i = start!; i <= end!; i++) {
      if (isInvalid(i)) {
        invalidIDs.push(i);
      }
    }
    console.log(start, end, invalidIDs);
  });
  console.log(invalidIDs.reduce((sum, v) => sum + v));
}

function partTwo(rotations: string[]) {
}

// const input = await fs.readFile("./import-sample.txt", "utf-8");
const input = await fs.readFile("./import.txt", "utf-8");

partOne(input.slice(0, -1).split(","));
// partTwo(input.split("\n"));
