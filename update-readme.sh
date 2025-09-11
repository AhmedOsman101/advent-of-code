#!/usr/bin/env bash

set -euo pipefail

# ---  Main script logic --- #
currentYear="$(date +%Y)"
previousYear="$((currentYear - 1))"

# Replace placeholder in Readme.md
sed -e "s|${previousYear}|${currentYear}|g" -i Readme.md
