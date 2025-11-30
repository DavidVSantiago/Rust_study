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

        String nomeArquivo = "livro.txt";
        String result;

        try {
            result = lerArquivoUTF8(nomeArquivo);
        } catch (IOException e) {
            System.out.println(e.getMessage());
            System.exit(1);
            return;
        }
        String ocorrencia = "Et";
        crono.marcarInicio();
        long qtd = contaOcorrenciasTexto(result, ocorrencia, true);
        crono.marcarFim();

        System.out.println(qtd + " palavras");
        System.out.println("O programa demorou " + crono.obterTempoSegundos() + " segundos!");
        System.out.println("O programa demorou " + crono.obterTempoMili() + " milisegundos!");
        System.out.println("O programa demorou " + crono.obterTempoNano() + " nanosegundos!");
    }

    // === Equivalente ao ler_arquivo_utf8 ===
    public static String lerArquivoUTF8(String nomeArquivo) throws IOException {
        return Files.readString(Paths.get(nomeArquivo), StandardCharsets.UTF_8);
    }

    // === Equivalente ao conta_palavras_texto ===
    public static long contaPalavrasTexto(String texto) {
        if (texto == null || texto.trim().isEmpty()) {
            return 0;
        }
        return texto.trim().split("\\s+").length;
    }

    // === Equivalente ao conta_ocorrencias_texto ===
    public static int contaOcorrenciasTexto(String textoCompleto,
                                        String ocorrencia,
                                        boolean ignorarCase) {

        if (textoCompleto == null || ocorrencia == null || ocorrencia.isEmpty()) {
            return 0;
        }

        if (ignorarCase) {
            textoCompleto = textoCompleto.toLowerCase();
            ocorrencia = ocorrencia.toLowerCase();
        }

        int count = 0;
        int index = 0;

        while ((index = textoCompleto.indexOf(ocorrencia, index)) != -1) {
            count++;
            index += ocorrencia.length(); // comportamento igual ao matches: não sobrepõe
        }

        return count;
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
