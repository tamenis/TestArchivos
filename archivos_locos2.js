const fs = require('fs');
const path = require('path');
const readline = require('readline');

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

async function copiarArchivo(origen, destino) {
    const contenido = await fs.promises.readFile(origen);
    await fs.promises.writeFile(destino, contenido);
}

async function eliminarArchivo(destino) {
    await fs.promises.unlink(destino);
}

async function obtenerArgumentos() {
    return new Promise((resolve, reject) => {
        rl.question('Ingrese la ruta del archivo: ', archivo => {
            rl.question('Ingrese la carpeta de destino: ', carpetaDestino => {
                rl.question('Ingrese el número de copias: ', numCopias => {
                    rl.question('Ingrese el tiempo entre copias (en segundos): ', tiempoEntreCopias => {
                        rl.question('Ingrese el número de repeticiones: ', repeticiones => {
                            resolve({ archivo, carpetaDestino, numCopias: parseInt(numCopias), tiempoEntreCopias: parseInt(tiempoEntreCopias), repeticiones: parseInt(repeticiones) });
                        });
                    });
                });
            });
        });
    });
}

async function main() {
    // Obtener argumentos del usuario
    const { archivo, carpetaDestino, numCopias, tiempoEntreCopias, repeticiones } = await obtenerArgumentos();

    // Verificar si el archivo existe
    if (!fs.existsSync(archivo)) {
        console.log(`El archivo ${archivo} no existe.`);
        rl.close();
        return;
    }

    // Verificar si la carpeta de destino existe, si no, crearla
    if (!fs.existsSync(carpetaDestino)) {
        fs.mkdirSync(carpetaDestino, { recursive: true });
    }

    // Realizar las copias y eliminarlas después de un tiempo
    for (let r = 0; r < repeticiones; r++) {
        const copias = [];
        
        // Copiar los archivos de forma asíncrona
        for (let i = 0; i < numCopias; i++) {
            const destino = path.join(carpetaDestino, `${i}_${path.basename(archivo)}`);
            copias.push(copiarArchivo(archivo, destino));
            console.log(`Copia ${i} creada en ${destino}`);
        }

        // Esperar a que todas las copias se completen
        await Promise.all(copias);

        // Esperar el tiempo entre copias
        await new Promise(resolve => setTimeout(resolve, tiempoEntreCopias * 1000));

        // Eliminar las copias después del tiempo especificado
        for (let i = 0; i < numCopias; i++) {
            const destino = path.join(carpetaDestino, `${i}_${path.basename(archivo)}`);
            eliminarArchivo(destino);
            console.log(`Copia ${i} eliminada`);
        }
    }

    rl.close();
}

main().catch(err => console.error(err));
