function partOne() {
  try {
    const input = Deno.readTextFileSync("input.txt").trim().split("\n");
    let total = 0;

    for (const line of input) {
      const dimensions = line.split("x").map(x => Number.parseInt(x));
      const [l, w, h] = dimensions;
      dimensions.sort((a, b) => a - b);
      const [min_1, min_2] = dimensions;

      total += 2 * l * h + 2 * l * w + 2 * h * w + min_1 * min_2;
    }
    console.log(total);
  } catch (e) {
    console.error(`An Error happened: ${(e as Error).message}`);
  }
}

function partTwo() {
  try {
    const input = Deno.readTextFileSync("input.txt");
  } catch (e) {
    console.error(`An Error happened: ${(e as Error).message}`);
  }
}

partOne();
partTwo();
