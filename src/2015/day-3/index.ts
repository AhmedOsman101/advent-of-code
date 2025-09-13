type Direction = "^" | "v" | "<" | ">";

function partOne() {
  try {
    const input = Deno.readTextFileSync("input.txt").split("") as Direction[];
    const coords_set = new Set<string>();
    const coords = [0, 0];
    coords_set.add(coords.join(","));

    for (const char of input) {
      switch (char) {
        case "^":
          coords[1] += 1;
          break;
        case "v":
          coords[1] -= 1;
          break;
        case "<":
          coords[0] -= 1;
          break;
        case ">":
          coords[0] += 1;
          break;
      }

      coords_set.add(coords.join(","));
    }

    console.log(coords_set.size);
  } catch (e) {
    console.error(`An Error happened: ${(e as Error).message}`);
  }
}

function partTwo() {
  try {
    const input = Deno.readTextFileSync("input.txt").split("") as Direction[];
    const coords_set = new Set<string>();
    const santaCoords = [0, 0];
    const robotCoords = [0, 0];
    coords_set.add(santaCoords.join(","));

    input.forEach((char, turn) => {
      const coords = turn % 2 === 0 ? santaCoords : robotCoords;
      switch (char) {
        case "^":
          coords[1] += 1;
          break;
        case "v":
          coords[1] -= 1;
          break;
        case "<":
          coords[0] -= 1;
          break;
        case ">":
          coords[0] += 1;
          break;
      }
      coords_set.add(coords.join(",")); // Adds the same reference, ends up with one values
    });

    console.log(coords_set.size);
  } catch (e) {
    console.error(`An Error happened: ${(e as Error).message}`);
  }
}

partOne();
partTwo();
