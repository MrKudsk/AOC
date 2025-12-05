
import fs from "fs/promises";

const START = 50;

function partOne(rotations: string[]) {
  let current = START;
  let password = 0;

  rotations.forEach((rotation) => {
    const dir = rotation.slice(0, 1);
    const val = Number(rotation.slice(1));

    if (dir === "L") {
      current = (current - val + 100) % 100;
    } else {
      current = (current + val) % 100;
    }
    // console.log(dir, val, current);
    if (current === 0) password += 1;
  });
  console.log("PartOne:", password);
}

// const input = await fs.readFile("./import-sample.txt", "utf-8");
const input = await fs.readFile("./import.txt", "utf-8");

partOne(input.split("\n"));
