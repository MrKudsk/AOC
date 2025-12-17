
import fs from "fs/promises";

function neighbors(grid: string[], row: number, col: number) {
  return [
    grid[row - 1]?.[col - 1],
    grid[row - 1]?.[col],
    grid[row - 1]?.[col + 1],
    grid[row]?.[col - 1],
    grid[row]?.[col + 1],
    grid[row + 1]?.[col - 1],
    grid[row + 1]?.[col],
    grid[row + 1]?.[col + 1],
  ].reduce((sum, nb) => nb === "@" ? sum + 1 : sum, 0);
}

function partOne(grid: string[]) {
  let total = 0;
  const width = grid[0]!.length;
  for (let row = 0; row < grid.length; row++) {
    for (let col = 0; col < width; col++) {
      if (grid[row]![col] === "@") {
        if (neighbors(grid, row, col) < 4) {
          total++;
        }
      }
    }
  }
  console.log("PartOne", total);
}

function partTwo(grid: string[]) {
  let total = 0;
  function remove() {
    const width = grid[0]!.length;
    let newgrid: string[] = [];
    let removed = 0;

    for (let row = 0; row < grid.length; row++) {
      let updateRow = "";
      for (let col = 0; col < width; col++) {
        let cell = grid[row]![col];
        if (grid[row]![col] === "@") {
          if (neighbors(grid, row, col) < 4) {
            removed++;
            cell = ".";
          }
        }
        updateRow += cell;
      }
      newgrid[row] = updateRow;
    }
    // console.log(newgrid);
    grid = newgrid;
    return removed;
  }

  let tmp = remove();
  while (tmp > 0) {
    total += tmp;
    tmp = remove();
  }

  console.log("PartTwo", total);
}

// const input = await fs.readFile("./import-sample.txt", "utf-8");
const input = await fs.readFile("./import.txt", "utf-8");

// partOne(input.slice(0, -1).split("\n"));
partTwo(input.slice(0, -1).split("\n"));
