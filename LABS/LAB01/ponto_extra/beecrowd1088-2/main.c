#include <stdio.h>
#include <stdlib.h>

int quantidade_trocas_impar(int *vec, int num_elements);
void swap(int *a, int *b);

char input[610000];
int to_be_sorted[100100];

int main() {
    int num_elements;

    char * pEnd;
    while (gets(input) && (num_elements = strtol(input, &pEnd, 10)) && num_elements != 0) {
        for (int i = 0; i < num_elements; i++) {
            to_be_sorted[i] = strtol(pEnd, &pEnd, 10);
        }

        if (quantidade_trocas_impar(to_be_sorted, num_elements)) {
            printf("Marcelo\n");
        } else {
            printf("Carlos\n");
        }

    }

    return 0;
}

int quantidade_trocas_impar(int *vec, int num_elements) {
    int contador = 0;

    for (int pos = 0; pos < num_elements; pos++) {
        int expected_number = pos + 1;
        int prev_original;

        while (vec[pos] != expected_number) {
            prev_original = vec[pos];
            swap(&vec[pos], &vec[prev_original - 1]);
            contador++;
        }
    }

    return contador & 1 == 1;
}

void swap(int *a, int *b) {
    int temp = *a;
    *a = *b;
    *b = temp;
}
