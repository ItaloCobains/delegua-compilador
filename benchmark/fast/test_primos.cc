#include <iostream>
using namespace std;

bool ehPrimo(int n) {
    if (n < 2) return false;
    if (n == 2) return true;
    if (n % 2 == 0) return false;
    int limite = n / 2;
    for (int i = 3; i <= limite; i++) {
        if (n % i == 0) return false;
    }
    return true;
}

int contarPrimos(int limite) {
    int contador = 0;
    for (int numero = 2; numero <= limite; numero++) {
        if (ehPrimo(numero)) contador++;
    }
    return contador;
}

int main() {
    int limite = 1000000;
    int resultado = contarPrimos(limite);
    cout << "RESULT:" << resultado << endl;
    return 0;
}
