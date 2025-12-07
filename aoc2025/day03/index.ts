
import fs from "fs/promises";

function partOne(banks: string[]) {
  let total = 0;
  banks.forEach((bank) => {
    let firstIdx = 0;
    for (let i = 0; i < bank.length - 1; i++) {
      if (bank[i]! > bank[firstIdx]!) firstIdx = i;
    }
    let secoundIdx = firstIdx + 1;
    for (let i = secoundIdx; i < bank.length; i++) {
      if (bank[i]! > bank[secoundIdx]!) secoundIdx = i;
    }

    const joltage: number = bank[firstIdx]! * 10 + bank[secoundIdx]! * 1;
    total += joltage;
  });
  console.log("PartOne", total);
}

function partTwo(banks: string[]) {
  let total = 0;

  banks.forEach((bank) => {
    let joltageStr: string[] = [];
    let lastIdx = 0;
    for (let d = 0; d < 12; d++) {
      let idx = lastIdx;
      for (let i = idx; i < bank.length - (11 - d); i++) {
        if (bank[i]! > bank[idx]!) idx = i;
      }
      joltageStr[d] = bank[idx]!;
      lastIdx = idx + 1;
    }
    // console.log(bank, Number(joltageStr.join("")));
    const joltage = Number(joltageStr.join(""));
    total += joltage;
  });
  console.log("PartTwo", total);
}

// const input = await fs.readFile("./import-sample.txt", "utf-8");
const input = await fs.readFile("./import.txt", "utf-8");

partOne(input.slice(0, -1).split("\n"));
partTwo(input.slice(0, -1).split("\n"));
