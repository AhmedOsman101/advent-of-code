# Tasks for Advent of Code

## download (year) (day)

> Downloads the input and puzzle for the given day and year.

It's recommended to have a `src/` directory for better code organization

```bash
[[ -d "./src" ]] && cd src
mkdir -p "${year}/day-${day}" &>/dev/null
cd "${year}/day-${day}"
aoc download --year "${year}" --day "${day}" --input-file input.txt
```

## submit (answer)

> Submits your solution answer for the specified year, day, and part of Advent of Code.

Handles adding the correct `--year`, `--day` and `--part` flags for the CLI automatically.

**OPTIONS**

- part
  - flags: -p --part
  - type: number
  - choices: 1, 2
  - desc: Puzzle part
  - required
- year
  - flags: -y --year
  - type: number
  - desc: Puzzle year (default: year of current or last Advent of Code event)
- day
  - flags: -d --day
  - type: number
  - desc: Puzzle day (default: last unlocked day during Advent of Code month)

```bash
cmd=('aoc' 'submit')
[[ -n "${year}" ]] && cmd+=('--year' "${year}")
[[ -n "${day}" ]] && cmd+=('--day' "${day}")
cmd+=("${part}" "${answer}")

"${cmd[@]}"
```

## view (year) (day)

> Preview the puzzle for the given day and year using `frogmouth`.

```bash
[[ -d "src" ]] && cd src
frogmouth "./year${year}/day${day}/puzzle.md"
```
