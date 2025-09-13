# Tasks for Advent of Code

## download (year) (day)

> Downloads the input and puzzle for the given day and year.

It's recommended to have a `src/` directory for better code organization

```bash
[[ -d "src" ]] && cd src
# use ed editor, start at line 0 (before line 1), `i` -> enter insert mode
# write the contents, `.` -> ends insert mode, `wq` -> write and quit
yearMod="pub mod year${year};"

grep -q "${yearMod}" main.rs &>/dev/null || ed -s main.rs <<EOF
0i
${yearMod}
.
wq
EOF

mkdir -p "year${year}/day${day}" &>/dev/null

cd "year${year}"
dayMod="pub mod day${day};"
grep -q "${dayMod}" mod.rs &>/dev/null || echo "${dayMod}" >> mod.rs

cd "day${day}"

[[ -f 'mod.rs' ]] || cat "$(git rev-parse --show-toplevel)/templates/day.rs" > mod.rs

sed -i -e "s|{{YEAR}}|${year}|g" -e "s|{{DAY}}|${day}|g" mod.rs

aoc download --overwrite --year "${year}" --day "${day}" --input-file input.txt
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
