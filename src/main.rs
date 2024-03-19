use std::env;
use std::fs;
use std::path::Path;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::copy;
use tokio::time;

async fn copiar_archivo(origen: String, destino: String) {
    let mut file = File::open(origen).await.expect("Error al abrir el archivo de origen");
    let mut destino_file = File::create(destino.clone()).await.expect("Error al crear el archivo de destino");
    copy(&mut file, &mut destino_file).await.expect("Error al copiar el archivo");
}

async fn eliminar_archivo(destino: String) {
    fs::remove_file(&destino).expect("Error al eliminar la copia");
}

#[tokio::main]
async fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 6 {
        println!("Uso: {} <archivo> <carpeta_destino> <num_copias> <tiempo_entre_copias> <repeticiones>", args[0]);
        return;
    }
    let archivo = args[1].clone();
    let carpeta_destino = args[2].clone();
    let num_copias: u32 = args[3].parse().expect("El número de copias debe ser un entero");
    let tiempo_entre_copias: u64 = args[4].parse().expect("El tiempo entre copias debe ser un entero");
    let repeticiones: u32 = args[5].parse().expect("El número de repeticiones debe ser un entero");

    // Verificar si el archivo existe
    if !Path::new(&archivo).exists() {
        println!("El archivo {} no existe.", archivo);
        return;
    }

    // Verificar si la carpeta de destino existe, si no, crearla
    if !Path::new(&carpeta_destino).exists() {
        fs::create_dir(&carpeta_destino).expect("Error al crear la carpeta de destino");
    }

    // Realizar las copias y eliminarlas después de un tiempo
    for _ in 0..repeticiones {
        let mut copias = Vec::new();
        
        // Copiar los archivos de forma asíncrona
        for i in 0..num_copias {
            let origen = archivo.clone();
            let destino = format!("{}/{}_{}", carpeta_destino, i, Path::new(&archivo).file_name().unwrap().to_str().unwrap());
            let futura_copia = Box::pin(copiar_archivo(origen, destino));
            copias.push(futura_copia);
        }

        // Esperar a que todas las copias se completen
        for copia in &mut copias {
            copia.await;
        }

        // Esperar el tiempo entre copias
        time::sleep(Duration::from_secs(tiempo_entre_copias)).await;

        // Eliminar las copias después del tiempo especificado
        let mut tareas_eliminacion = Vec::new();
        for i in 0..num_copias {
            let destino = format!("{}/{}_{}", carpeta_destino, i, Path::new(&archivo).file_name().unwrap().to_str().unwrap());
            let futura_eliminacion = Box::pin(eliminar_archivo(destino.clone()));
            tareas_eliminacion.push(futura_eliminacion);
            println!("Copia {} para eliminar", i);
        }

        // Esperar a que todas las tareas de eliminación se completen
        for tarea in tareas_eliminacion {
            tarea.await;
        }
    }
}
