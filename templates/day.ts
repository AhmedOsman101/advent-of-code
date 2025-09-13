function partOne() {
  try {
    const input = Deno.readTextFileSync("input.txt");
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
