#include <stdio.h>
#include <stdlib.h>

int quantidade_trocas_impar(int *vec, int num_elements);
void zera_e_passa(int vec[], int pos, int* contador, int expected_number);

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

int quantidade_trocas_impar(int vec[], int num_elements) {
    int contador = 0;

    for (int pos = 0; pos < num_elements; pos++) {
        zera_e_passa(vec, pos, &contador, pos + 1);
    }

    return contador & 1 == 1;
}

void zera_e_passa(int vec[], int pos, int* contador, int expected_number) {
    printf("pos = %d, vec[pos] = %d, expected = %d\n", pos, vec[pos], expected_number);
    printf("contador: %d\n", *contador);
    sleep(1);
    if (vec[pos] != expected_number && vec[pos] != 0) {
        (*contador)++;
        zera_e_passa(vec, vec[pos]-1, contador, expected_number);
        vec[pos] = 0;        
    }
}
