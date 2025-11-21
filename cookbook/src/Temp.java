import java.lang.reflect.Array;
import java.util.Arrays;
import java.util.Random;

    public class Temp {
 
    private static final byte[] N_ARR = {48, 49, 50, 51, 52, 53, 54, 55, 56, 57};
    private static final byte[] LMAI_ARR = {65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90};
    private static final byte[] LMIN_ARR = {97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122};
    private static final byte[] L_ARR = {65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122};
    private static final byte[] NL_ARR = {48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122};
    private static final byte[] E_ARR = {33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 58, 59, 60, 61, 62, 63, 64, 91, 92, 93, 94, 95, 96, 123, 124, 125, 126};
    private static final byte[] T_ARR = {33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126};

    public static String rand_senha(int tam, TipoSenha t){
        byte[] caracteres_permitidos;
        switch(t) {
            case TipoSenha.NUMEROS: caracteres_permitidos = N_ARR; break;
            case TipoSenha.LETRAS_MAIUSCULAS: caracteres_permitidos = LMAI_ARR; break;
            case TipoSenha.LETRAS_MINUSCULAS: caracteres_permitidos = LMIN_ARR; break;
            case TipoSenha.LETRAS: caracteres_permitidos = L_ARR; break;
            case TipoSenha.NUMEROS_LETRAS: caracteres_permitidos = NL_ARR; break;
            case TipoSenha.ESPECIAIS: caracteres_permitidos = E_ARR; break;
            case TipoSenha.TOTAL: caracteres_permitidos = T_ARR; break;
            default: caracteres_permitidos = T_ARR;
        };
        

        int c_tam = caracteres_permitidos.length; // a quantidade de caracteres no array de caracteres permitidos

        char[] senha = new char[tam]; // um array de tamanho t
        // percorre cada caracter para colocar os aleatórios
        for (int i=0;i<senha.length;i++){
            // obtem um indice dos caractere aleatórios permitidos
            int indice = rand(0, c_tam-1);
            senha[i] = (char)caracteres_permitidos[indice];
        }
        // converte para string e retorna
        return new String(senha);
    }

    private static final Random rng = new Random();
    public static int rand() {
        Random rng = new Random(); 
        return rng.nextInt(); 
    }
    /** Retorna um número aleatório no intervalo [a,b] inclusivo*/
    public static int rand(int a, int b) {
        int min = Math.min(a, b);
        int max = Math.max(a, b);
        Random rng = new Random();
        return rng.nextInt(max - min + 1) + min;
    }
    public enum TipoSenha {
        NUMEROS, 
        LETRAS_MAIUSCULAS, 
        LETRAS_MINUSCULAS, 
        LETRAS, 
        NUMEROS_LETRAS, 
        ESPECIAIS, 
        TOTAL
    }

    public static void heapSort(int[] A){
        int size = A.length;
        // construção do HEAP MÁXIMO
        int p=(size/2)-1; // obtém o último pai
        while (p>=0) {
            siftDown(A,p,size);
            p--;
        }  

        // isola o maior valor no final do array e reaproveita o heap
        while(size>1){
            swap(A, 0, --size);
            siftDown(A, 0, size);
        }
    }

    public static void siftDown(int[] A, int p, int size) {
        while (p*2+1 < size) {
            int f = p*2+1;
            if (f+1 < size && A[f+1] > A[f]) f++;
            if (A[f] > A[p]) swap(A,f,p);
            else break;
            p = f;
        }
    }

    public static void swap(int[] A, int p1, int p2){
        int aux = A[p1];
        A[p1]=A[p2];
        A[p2]=aux;
    }

    public static void main(String[] args) {

        int[] A = new int[10];
        for (int i=0;i<A.length;i++) {
            A[i] = rand(0, 1000);
        }

        System.out.print("Array antes: ");
        for (int a : A){System.out.print(a+",");}   
        long start = System.nanoTime();
        Arrays.sort(A);
        long end = System.nanoTime();
        long elapsed = (end-start);
        System.out.print("\nArray depois: ");
        for (int a : A){System.out.print(a+",");}   
        System.out.println("\nTempo total gasto: "+elapsed+" ns");
    }
}
