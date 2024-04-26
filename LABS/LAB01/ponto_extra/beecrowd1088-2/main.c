#include <stdio.h>
#include <stdlib.h>

int quantidade_trocas_impar(int *vec, int num_elements);

char input[610000];
int to_be_sorted[100100];

int main() {

    setvbuf(stdout, NULL, _IOFBF, BUFSIZ);

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

    fflush(stdout);

    return 0;
}

int quantidade_trocas_impar(int *vec, int num_elements) {
    int contador = 0;

    for (int pos = 0; pos < num_elements; pos++) {
        int expected_number = pos + 1;

        int temp;

        while (vec[pos] != expected_number) {
               
            temp = vec[vec[pos] - 1];
            vec[vec[pos] - 1] = vec[pos];
            contador++;

            if (temp != expected_number) { 
                vec[pos] = vec[temp - 1];
                vec[temp - 1] = temp;
                contador++;
            } else {
                break;
            }
        }
    }

    return contador & 1 == 1;
}

// Por algum motivo, diminuir o numero de swaps nao parece ter afetado a performance, não sei se o compilador ja fazia isso.
// versão antiga é mais rápida, mesmo contendo 50% mais movimentação de dados no código fonte.

