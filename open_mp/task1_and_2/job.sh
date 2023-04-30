#!/bin/bash
#SBATCH -o ./ans.txt # STDOUT
#SBATCH -c 12
#SBATCH -p gnu
#SBATCH --thread-spec=12
./task2
