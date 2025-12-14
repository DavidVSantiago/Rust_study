import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Paths;

public class Main {

    public static void main(String[] args) {
        new Main();
    }

    public Main(){
        Cronometro crono = new Cronometro();

        int num = 10_000_000;
        int cont=0;
        crono.marcarInicio();
        for(int i=1;i<num;i++){
            if (calc_primo(i)) cont+=1;
        }
        crono.marcarFim();
        System.out.println("Entre 1 e "+num+" existem "+cont+" numeros primos");
        System.out.println("Processo v1 realizado em "+crono.obterTempoSegundos()+" segundos!");
    }

    public boolean calc_primo(int num){
        if (num <= 3) { return num > 1; } // false p/ 1 e true p/ 2 ou 3
        if (num%2 == 0 || num%3 == 0) {return false; } // falso p os multiplos de 2 ou 3
        int sup = (int)Math.sqrt(num);
        // Otimização 6k +/- 1
        int i = 5; // começa a verificar a partir de 5, pois já verificou 2 e 3
        while (i <= sup){
            if (num%i==0 || num%(i+2)==0) {return  false;}
            i+=6; // salta os multiplos de 2 e 3
        }
        return true;
    }

    public class Cronometro {

        private long inicio;
        private long fim;

        public void marcarInicio() {
            inicio = System.nanoTime();
        }

        public void marcarFim() {
            fim = System.nanoTime();
        }

        public long obterTempoNano() {
            return fim - inicio;
        }

        public double obterTempoMili() {
            return (fim - inicio) / 1_000_000.0;
        }

        public double obterTempoSegundos() {
            return (fim - inicio) / 1_000_000_000.0;
        }
    }

}
