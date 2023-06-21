#!/bin/bash
#SBATCH -p gnu
#SBATCH -N 1 
#SBATCH -n 8
#SBATCH --ntasks-per-node=8
mpirun xhpl
