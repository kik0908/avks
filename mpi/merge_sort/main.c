#include "mpi.h"
#include <stdio.h>
#include <stdlib.h>

void merge(int* arr, int l, int m, int r) {
    
    int f_n = m - l + 1;
    int s_n = r - m;

    int* left = (int*)malloc(f_n * sizeof(int));
    int* right = (int*)malloc(s_n * sizeof(int));

    for (int i = 0; i < f_n; ++i){
        left[i] = arr[l + i];
    }

    for (int j = 0; j < s_n; ++j){
        right[j] = arr[m + 1 + j];
    }
    int i = 0;
    int j = 0;
    i = 0;
    j = 0;

    while (i < f_n && j < s_n) {

        if (left[i] > right[j]) {
            arr[l] = right[j];
            ++j;
        }
        else {
            arr[l] = left[i];
            ++i;
        }
        ++l;
    }

    while (i < f_n) {
        arr[l] = left[i];
        ++i;
        ++l;
    }

    while (j < s_n) {
        arr[l] = right[j];
        ++j;
        ++l;
    }

    free(right);
    free(left);
}

void mergeSort(int* arr, int l, int r)
{
    if (l < r) {
        int m = l + (r - l) / 2;

        mergeSort(arr, l, m);
        mergeSort(arr, m + 1, r);

        merge(arr, l, m, r);
    }
}

int main() {
    MPI_Init(NULL, NULL);

    int n = 5000;

    int size, rank;
    int* array = (int*)malloc(n * sizeof(int));

    MPI_Comm_size(MPI_COMM_WORLD, &size);
    MPI_Comm_rank(MPI_COMM_WORLD, &rank);

    int per_sub_arr = n / size;

    MPI_Barrier(MPI_COMM_WORLD);

    int* sub_arr = (int*)malloc(per_sub_arr * sizeof(int));

    double start = MPI_Wtime();

    MPI_Scatter(array, per_sub_arr, MPI_INT, sub_arr, per_sub_arr, MPI_INT, 0, MPI_COMM_WORLD);

    mergeSort(sub_arr, 0, per_sub_arr - 1);

    int* sorted_array;
    if (rank == 0) {
        sorted_array = (int*)malloc(n * sizeof(int));
    }

    MPI_Gather(sub_arr, per_sub_arr, MPI_INT, sorted_array, per_sub_arr, MPI_INT, 0, MPI_COMM_WORLD);

    if (rank == 0) {
        mergeSort(sorted_array, 0, n - 1);
        double finish = MPI_Wtime();

        for (int i = 1; i < size; ++i) {
            if (sorted_array[i] < sorted_array[i - 1]) {
                MPI_Barrier(MPI_COMM_WORLD);
                MPI_Finalize();
                printf("Bad result\n");
                return 1;
            }
        }

        printf("Time: %.2f(ms)\n", (finish - start) * 1000);
    }

    MPI_Barrier(MPI_COMM_WORLD);
    MPI_Finalize();

    return 0;
}
