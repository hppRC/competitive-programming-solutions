#!/bin/bash

bash ./utility/organize.sh
bash ./utility/count.sh > README.md
git add -A
git commit -m "add some solutions"
git push origin master