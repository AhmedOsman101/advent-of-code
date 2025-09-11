#!/usr/bin/env bash

set -eo pipefail

trap 'exit 1' SIGUSR1

# ---  Main script logic --- #
currentYear="$(date +%Y)"
previousYear="$((currentYear - 1))"
langPlaceholder="{{LANGUAGE}}"

lang="${1:-$langPlaceholder}"

# Replace placeholders in Readme.md
sed -i Readme.md \
  -e "s|${previousYear}|${currentYear}|g" \
  -e "s|${langPlaceholder}|${lang}|g"
