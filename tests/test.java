import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);

        int num1 = scanner.nextInt();

        int num2 = scanner.nextInt();

        int sum = num1 + num2;
        System.out.printf("%d\n", sum);
        System.out.printf("%d", sum);

        scanner.close();
    }
}
