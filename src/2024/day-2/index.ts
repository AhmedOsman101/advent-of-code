function isUnsafe(isAscending: boolean, prev: number, curr: number) {
  const distance = Math.abs(prev - curr);
  return (
    (isAscending && curr < prev) ||
    (!isAscending && prev < curr) ||
    distance < 1 ||
    3 < distance
  );
}

function checkLevel(level: number[]): boolean {
  const len = level.length - 1;
  const distances: number[] = [];
  let positivesCount = 0;
  let negativesCount = 0;

  for (let i = 0; i < len; i++) {
    const diff = level[i] - level[i + 1];
    if (diff === 0) return false;
    distances[i] = Math.abs(diff);
    diff < 0 ? negativesCount++ : positivesCount++;
  }

  const isAscending = negativesCount > positivesCount;

  for (let i = 1; i < level.length; i++) {
    const prev = level[i - 1];
    const curr = level[i];

    /*
     * In descending order, previous should be smaller than current
     * In descending order, previous should be bigger than current
     * distance between each number should be between 1 and 3 (inclusive)
     */
    if (isUnsafe(isAscending, prev, curr)) return false;
  }

  return true;
}

function partOne() {
  try {
    const lines = Deno.readTextFileSync("input.txt").trim().split("\n");
    let totalSafe = 0;
    for (const report of lines) {
      const level = report.split(" ").map(Number);
      if (checkLevel(level)) totalSafe++;
    }

    console.log(totalSafe);
  } catch (e) {
    console.error(`An Error happened: ${(e as Error).message}`);
  }
}

function checkLevelWithDampener(level: number[], tried = false): boolean {
  const len = level.length - 1;
  const distances: number[] = [];
  let positivesCount = 0;
  let negativesCount = 0;

  for (let i = 0; i < len; i++) {
    const diff = level[i] - level[i + 1];
    distances[i] = Math.abs(diff);
    diff < 0 ? negativesCount++ : positivesCount++;
  }

  const isAscending = negativesCount > positivesCount;

  for (let i = 0; i < len; i++) {
    const prev = level[i];
    const curr = level[i + 1];

    /*
     * In descending order, previous should be smaller than current
     * In descending order, previous should be bigger than current
     * distance between each number should be between 1 and 3 (inclusive)
     */
    if (isUnsafe(isAscending, prev, curr)) {
      if (tried) return false;
      if (
        !checkLevelWithDampener(level.toSpliced(i, 1), true) &&
        !checkLevelWithDampener(level.toSpliced(i + 1, 1), true)
      ) {
        return false;
      }
    }
  }

  return true;
}

function partTwo() {
  try {
    const lines = Deno.readTextFileSync("input.txt").trim().split("\n");
    let totalSafe = 0;
    for (const report of lines) {
      const level = report.split(" ").map(Number);
      if (checkLevelWithDampener(level)) totalSafe++;
    }

    console.log(totalSafe);
  } catch (e) {
    console.error(`An Error happened: ${(e as Error).message}`);
  }
}

partOne();
partTwo();
