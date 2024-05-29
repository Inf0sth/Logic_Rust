mod ops;

use rand::Rng;

fn main() {
    println!("Hello, world!{:?}", encrypt());    decrypt();
}

fn encrypt() ->u32{
    let mut rng = rand::thread_rng();
    let mut matrix: [u32; 9] = [0; 9];
    for val in 0..8{
        matrix[val] = rng.gen_range(100..=999);
    }
    let key = ops::determinant(&matrix);
    return key;
}


/*
def cifrar(message):
    # Creacion de la matriz cuadrada principal
    x = 1.5 * (float(len(message))) # Multiplicamos por 1.5 para no tener sobrantes
    er_r = 0.0001  # Margen de error tolerable
    y = x / 2  # Suposición inicial

    while abs(y * y - x) >= er_r:
        y = (y + x / y) / 2  # Cálculo de la nueva suposición

    # Y es la raiz de la longitud del mensaje multiplicado por 1.5
    y = round(y) # Obtenemos un valor redondeado de para trabajar valores exactos.
    print("La raíz cuadrada de", x, "es", y)

    array_to_fill = zeros((y, y))  # Creamos una nueva matriz de ceros.

    for i in range(y):
        for j in range(y):
            pos = i * y + j
            if pos < len(message):
                letra = message[pos]
                array_to_fill[i, j] = ord(letra)  # Convertir el carácter a su valor numérico (ASCII)
            else:
                break

    print("Matriz sin transponer:")
    print(array_to_fill)

    # Matriz transpuesta:
    array_transposed = zeros((y, y))  # Crear una matriz vacía para almacenar la transpuesta

    for i in range(y):
        for j in range(y):
            array_transposed[i, j] = array_to_fill[j, i]  # Asignar los elementos transpuestos

    print("Matriz transpuesta:")
    print(array_transposed)

    # Multiplicacion de matrices (transpuesta1 x det(matriz2)):

    for i in range(y):
        for j in range(y):
            array_transposed[i, j] *= det

    print("Matriz transpuesta multiplicada por el determinante:")
    print(array_transposed)
    
    # Obtenemos el cifrado en una lista:
    matriz2 = []
    # Iniciamos un ciclo que rivise la matriz anterior en cada posición:
    for i in range(y):
        for j in range(y):
            nums = array_transposed[i][j] # Pasamos un valor por cada posición a una variable momentaneamente
            matriz2 += [nums] # Pasamos los valores de la variable a la lista 2, sin más que una dimensión
    
    # Devolvemos la matriz
    print("\nLa matriz unidimensional es: ")
    # Convertimos la lista en matriz
    matriz2 = array(matriz2)
    print(matriz2)
*/

fn decrypt(){
    println!("Hello world!!");
}

/*
def descifrar():
    matriz2 = list(map(int, input("Ingrese los valores de la matriz unidimensional: ").split()))

    # Tamaño de la matriz cuadrada original
    y = int(sqrt(len(matriz2)))

    # Dividir la lista en sublistas de tamaño y
    sublists = [matriz2[i:i+y] for i in range(0, len(matriz2), y)]

    # Convertir las sublistas en una matriz
    array_transposed = array(sublists)

    # Dividir los valores por el determinante inverso
    det_inv = float(input("Ingrese el valor del determinante (clave): "))
    array_transposed = array_transposed / det_inv

    print("Matriz transpuesta multiplicada por el determinante:")
    print(array_transposed)

    # Obtener la matriz original invirtiendo la transposición
    array_to_fill = zeros((y, y))

    for i in range(y):
        for j in range(y):
            array_to_fill[i, j] = array_transposed[j, i]

    # Obtener el mensaje original a partir de la matriz
    original_message = ""

    for i in range(y):
        for j in range(y):
            num = array_to_fill[i, j]
            if num != 0:
                original_message += chr(int(num))

    print("El mensaje original es:", original_message)
*/
