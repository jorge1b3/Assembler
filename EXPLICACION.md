# Explicación del Proyecto 06

El código está escrito en Rust y actúa como un ensamblador para el lenguage HACK del proyecto 06 del libro nan2tetris. Este ensamblador toma código de ensamblaje como entrada y lo convierte en código de máquina.

1. **Definiciones**:
   - Define un enum llamado `Instruction` que representa diferentes tipos de instrucciones de ensamblaje: `Addressing`, `Computing` y `Labeling`.
   - Hay una implementación del trait `Display` para `Instruction` que permite formatear las instrucciones para la salida.
   - `VARIABLE_ADDRESS_START` se establece en 16, lo que representa la dirección de inicio para las variables en la memoria.

2. **Función Principal**:
   - La función `main` es el punto de entrada del programa. Procesa los argumentos de la línea de comandos y abre el archivo de entrada para su lectura.

3. **Manejo de Argumentos de la Línea de Comandos**:
   - Comprueba si se proporciona el número correcto de argumentos de la línea de comandos y si el archivo de entrada tiene una extensión `.asm`.

4. **Análisis de Instrucciones de Ensamblaje**:
   - El código lee el archivo de entrada línea por línea y procesa cada línea para crear un vector de objetos `Instruction`.
   - Recorta espacios en blanco al principio y al final y elimina comentarios (líneas que comienzan con `//`).
   - La función `parser` se utiliza para analizar las líneas en objetos `Instruction` según la estructura del lenguaje de ensamblaje.

5. **Construcción de la Tabla de Etiquetas**:
   - El código construye un `HashMap` llamado `label_table` para almacenar etiquetas y sus números de línea correspondientes.
   - Esta tabla se crea al iterar a través de las instrucciones y asociar las etiquetas con sus números de línea. Las etiquetas en sí no cuentan para el número de linea.

6. **Construcción de la Tabla de Símbolos**:
   - Se crea un `HashMap` llamado `symbol_table` para almacenar nombres de variables y sus direcciones de memoria.
   - Esta tabla se construye al iterar a través de las instrucciones y asociar las variables con direcciones de memoria.

7. **Conversión de Instrucciones a Código Binario**:
   - El código itera nuevamente a través de las instrucciones, esta vez convirtiéndolas a código binario de máquina.
   - La función `to_binary` traduce cada instrucción a su representación binaria utilizando tablas para las partes de la computación (`comp_table`), destino (`dest_table`) y salto (`jump_table`).
   - La función también maneja instrucciones de direccionamiento (por ejemplo, `@valor`), determinando si son constantes o variables.

8. **Escritura en el Archivo de Salida**:
   - Las instrucciones de código binario de máquina se escriben en un archivo de salida con una extensión `.hack`.

9. **Conclusión**:
    - El código ensambla con éxito el código de ensamblaje de entrada en código de máquina.
    - Maneja varios tipos de instrucciones, incluyendo direccionamiento (A-instructions), cálculos (C-instructions) y etiquetas (utilizadas para el flujo de control).
    - Gestiona variables y les asigna direcciones únicas.
    - El código de máquina resultante se escribe en un archivo de salida con el mismo nombre que el archivo de entrada, pero con una extensión `.hack`.

Este código realiza la función de un ensamblador, que es un componente esencial en el proceso de compilar lenguajes de programación de nivel superior en código de máquina para su ejecución en una computadora.
