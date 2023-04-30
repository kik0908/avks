#!/bin/bash
#SBATCH -o ./ans.txt # STDOUT
#SBATCH -c 10
#SBATCH -p gnu
#SBATCH --thread-spec=10
./zero_task
