#include <stdio.h>
#include <mpi.h>

int main() {
    MPI_Init(NULL, NULL);

    int rank, size;
    MPI_Comm_size(MPI_COMM_WORLD, &size);
    MPI_Comm_rank(MPI_COMM_WORLD, &rank);
    

    MPI_Barrier(MPI_COMM_WORLD);
    
    int n = 1e8;
    double local_sum = 0.0, pi = 0.0;

    double start = MPI_Wtime();
    
    for (int i = rank; i < n; i += size) {
        double tmp = (i + 0.5) / n;
        local_sum += 4.0 / (1 + tmp * tmp);
    }

    MPI_Reduce(&local_sum, &pi, 1, MPI_DOUBLE, MPI_SUM, 0, MPI_COMM_WORLD);

    MPI_Barrier(MPI_COMM_WORLD);
    double end = MPI_Wtime();

    if (rank == 0) {
        printf("PI: %.7f\n", pi / n);
        printf("Time in ms: %.2f\n", (end - start) * 1000);
    }  
    MPI_Finalize();
    return 0;
}
