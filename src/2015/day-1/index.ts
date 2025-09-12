function partOne() {
  try {
    const content = Deno.readTextFileSync("input.txt").trim();
    let floor = 0;
    for (const char of content) char === "(" ? floor++ : floor--;
    console.log(floor);
  } catch (e) {
    console.error(`An Error happened: ${(e as Error).message}`);
  }
}

function partTwo() {
  try {
    const content = Deno.readTextFileSync("input.txt").trim().split("");
    let floor = 0;
    content.forEach((char, position) => {
      char === "(" ? floor++ : floor--;
      if (floor === -1) {
        console.log(position + 1);
        Deno.exit(0);
      }
    });
  } catch (e) {
    console.error(`An Error happened: ${(e as Error).message}`);
  }
}
partTwo();

export { partOne, partTwo };
