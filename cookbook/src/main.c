#include <stdio.h>
#include <math.h>
#include <time.h>
#include <stdbool.h>

// Protótipo da função
bool calc_primo(int n);

int main() {
    int num = 10000000;
    int cont = 0;
    
    // Variáveis para medir o tempo
    clock_t inicio, fim;
    double tempo_gasto;

    // Marca o início (crono.marcarInicio)
    inicio = clock();

    for (int i = 1; i < num; i++) {
        if (calc_primo(i)) {
            cont += 1;
        }
    }

    // Marca o fim (crono.marcarFim)
    fim = clock();

    // Calcula a diferença e converte para segundos
    tempo_gasto = (double)(fim - inicio) / CLOCKS_PER_SEC;

    printf("Entre 1 e %d existem %d numeros primos\n", num, cont);
    printf("Processo v1 realizado em %.6f segundos!\n", tempo_gasto);

    return 0;
}

bool calc_primo(int n) {
    if (n <= 3) { return n > 1; };
    if (n%2 == 0 || n%3 == 0) {return false; }
    // Otimização: calcular a raiz quadrada para reduzir iterações
    int sup = (int)sqrt(n);
    for (int i = 5; i <= sup; i+=6) {
        if (n % i == 0 || n % (i + 2)==0) {return false;}
    }
    return true;
}