
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

  function findOverlap() {
    let overlap = false;
    rangesB = [];
    for (let j = 0; j < rangesA.length; j++) {
      let noOverlap = true;
      for (let i = 0; i < rangesB.length; i++) {
        if (rangesA[j].start <= rangesB[i].end && rangesA[j].end >= rangesB[i].start) {
          noOverlap = false;
          overlap = true;
          if (rangesA[j] < rangesB[i].start) rangesB[i].start = rangesA[j].start;
          if (rangesA[j].end > rangesB[i].end) rangesB[i].end = rangesA[j].end;
        }
      }
      // console.log(rangesA[j]);
      if (noOverlap) rangesB.push({ ...rangesA[j] });
    }
    // console.log("range B: ", rangesB, overlap);
    return overlap;
  }

  const ranges = grid![0].split("\n").map(line => line.split("-").map(Number));
  // console.log(ranges);
  let rangesA = [];
  let rangesB = [];
  // console.log(numbers);
  for (const r of ranges) {
    const start = r[0];
    const end = r[1];
    rangesA.push({ start, end });
  }


  while (findOverlap()) {
    // copy range b to range a
    rangesA = [];
    for (let i = 0; i < rangesB.length; i++) {
      rangesA.push({ ...rangesB[i] });
    }
  }

  // console.log("range A: ", rangesA);
  const total = rangesA.reduce((sum, r) => sum = sum + (r.end + 1 - r.start), 0);
  console.log("PartTwo", total);
}

// const input = await fs.readFile("./import-sample.txt", "utf-8");
const input = await fs.readFile("./import.txt", "utf-8");


// partOne(input.split("\n\n"));
partTwo(input.split("\n\n"));
