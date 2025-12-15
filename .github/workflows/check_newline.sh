#!/bin/bash

ERRORS=0

while IFS= read -r -d '' f
do
  if [[ $(tail -c 1 "$f") ]]; then
    echo "$f"
    ERRORS=1
  fi
done < <(git grep -Il -z '')

if [ $ERRORS -eq 1 ]; then
  echo "Files above have no newline at the end."
  exit 1
fi
