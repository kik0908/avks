#include <mpi.h>
#include <stdio.h>
#include <stdlib.h>

int main()
{
    MPI_Init(NULL, NULL);

    int n = 1000;

    int size, rank;
    MPI_Comm_size(MPI_COMM_WORLD, &size);
    MPI_Comm_rank(MPI_COMM_WORLD, &rank);

    int rows_for_one_proc = n / size;
    int first_row = rank * rows_for_one_proc;

    int last_row = first_row + rows_for_one_proc;

    if (rank == size - 1)
        last_row += n % size;

    int** A = (int**)malloc(n * sizeof(int*));
    int** B = (int**)malloc(n * sizeof(int*));
    int** C = (int**)malloc(n * sizeof(int*));
    int** C_single = (int**)malloc(n * sizeof(int*));

    for (int i = 0; i < n; ++i) {
        A[i] = (int*)malloc(n * sizeof(int));
        B[i] = (int*)malloc(n * sizeof(int));
        C[i] = (int*)malloc(n * sizeof(int));
        C_single[i] = (int*)malloc(n * sizeof(int));

    }

    double start = MPI_Wtime();

    for (int i = first_row; i < last_row; ++i) {
        for (int j = 0; j < n; ++j) {
            for (int k = 0; k < n; ++k) {
                C[i][j] += A[i][k] * B[k][j];
            }
        }
    }

    MPI_Barrier(MPI_COMM_WORLD);

    int over_rows = n % size;

    if (rank == 0) {
        for (int i = 1; i < size; ++i) {
            int tmp = rows_for_one_proc;
            if (i == size - 1) {
                tmp += over_rows;
            }

            for (int j = 0; j < tmp; ++j) {
                MPI_Recv(C[first_row + j], n, MPI_INT, i, 0, MPI_COMM_WORLD, MPI_STATUS_IGNORE);
            }
        }
    } 
    else {
        for (int i = first_row; i < last_row; ++i) {
            MPI_Send(C[i], n, MPI_INT, 0, 0, MPI_COMM_WORLD);
        }
    }

    MPI_Barrier(MPI_COMM_WORLD);
    double end = MPI_Wtime();

    if (rank == 0) {
        double time = end - start;
        start = MPI_Wtime();

		for (int i = 0; i < n; ++i) {
			for (int j = 0; j < n; ++j) {
                C_single[i][j] = 0;
				for (int k = 0; k < n; ++k) {
					C_single[i][j] += A[i][k] * B[k][j];
                }
            }
        }
		end = MPI_Wtime();

        printf("Time: %f(ms)\n", time * 1000);
        printf("Time single: %f(ms)\n", (end-start) * 1000);

        for (int i = 0; i < n; ++i) {
            for (int j = 0; j < n; ++j){
                if (C[i][j] != C_single[i][j]) {
                    printf("Not equal!!!!!!\n");
                    goto wrong;
                }
            }
        }

        printf("Equal!\n");

    }

    MPI_Finalize();
    
    return 0;
    wrong: return 1;
}
