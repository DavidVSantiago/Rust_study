#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <string.h>

// Definição dos arrays de caracteres (Valores ASCII)
const char N_ARR[] = {48, 49, 50, 51, 52, 53, 54, 55, 56, 57};
const char LMAI_ARR[] = {65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90};
const char LMIN_ARR[] = {97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122};
const char L_ARR[] = {65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122};
const char NL_ARR[] = {48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122};
const char E_ARR[] = {33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 58, 59, 60, 61, 62, 63, 64, 91, 92, 93, 94, 95, 96, 123, 124, 125, 126};
const char T_ARR[] = {33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126};

// Enumeração equivalente
typedef enum {
    NUMEROS,
    LETRAS_MAIUSCULAS,
    LETRAS_MINUSCULAS,
    LETRAS,
    NUMEROS_LETRAS,
    ESPECIAIS,
    TOTAL
} TipoSenha;

// Função auxiliar para gerar número aleatório no intervalo [a, b]
int rand_int(int a, int b) {
    if(a > b)
        return (rand() % (a - b + 1)) + b;
    else
        return (rand() % (b - a + 1)) + a;
}

// Função principal de geração de senha
char* rand_senha(int tam, TipoSenha t) {
    const char* caracteres_permitidos;
    int c_tam;

    // Seleciona o array correto e seu tamanho
    switch(t) {
        case NUMEROS: 
            caracteres_permitidos = N_ARR; 
            c_tam = sizeof(N_ARR); 
            break;
        case LETRAS_MAIUSCULAS: 
            caracteres_permitidos = LMAI_ARR; 
            c_tam = sizeof(LMAI_ARR); 
            break;
        case LETRAS_MINUSCULAS: 
            caracteres_permitidos = LMIN_ARR; 
            c_tam = sizeof(LMIN_ARR); 
            break;
        case LETRAS: 
            caracteres_permitidos = L_ARR; 
            c_tam = sizeof(L_ARR); 
            break;
        case NUMEROS_LETRAS: 
            caracteres_permitidos = NL_ARR; 
            c_tam = sizeof(NL_ARR); 
            break;
        case ESPECIAIS: 
            caracteres_permitidos = E_ARR; 
            c_tam = sizeof(E_ARR); 
            break;
        case TOTAL: 
            caracteres_permitidos = T_ARR; 
            c_tam = sizeof(T_ARR); 
            break;
        default: 
            caracteres_permitidos = T_ARR; 
            c_tam = sizeof(T_ARR);
    }

    // Aloca memória para a senha (+1 para o caractere nulo \0)
    char* senha = (char*)malloc((tam + 1) * sizeof(char));
    if (senha == NULL) {
        printf("Erro de alocação de memória.\n");
        exit(1);
    }

    for (int i = 0; i < tam; i++) {
        int indice = rand_int(0, c_tam - 1);
        senha[i] = caracteres_permitidos[indice];
    }

    senha[tam] = '\0'; // Finaliza a string
    return senha;
}

int main() {
    // Inicializa a semente do gerador aleatório (apenas uma vez)
    srand(time(NULL));

    struct timespec start, end;

    // Captura o tempo inicial
    clock_gettime(CLOCK_MONOTONIC, &start);

    // Gera a senha
    char* senha = rand_senha(100, TOTAL);

    // Captura o tempo final
    clock_gettime(CLOCK_MONOTONIC, &end);

    // Calcula o tempo decorrido em nanossegundos
    long seconds = end.tv_sec - start.tv_sec;
    long nanoseconds = end.tv_nsec - start.tv_nsec;
    long elapsed = seconds * 1000000000L + nanoseconds;

    printf("Senha gerada: %s\n", senha);
    printf("Tempo total gasto: %ld ns\n", elapsed);

    // Libera a memória alocada pelo malloc
    free(senha);

    return 0;
}