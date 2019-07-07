#!/bin/bash

bash organize.sh
bash count.sh > README.md
git add -A
git commit -m "add some solutions"
git push origin master