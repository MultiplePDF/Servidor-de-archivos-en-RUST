use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;
use std::time::{Duration, SystemTime};

fn main() -> io::Result<()> {
    // Carpeta donde se almacenarán los lotes
    let carpeta_lotes = Path::new("/ruta/a/la/carpeta/de/lotes");

    // Validar los archivos y guardar el lote en la carpeta de lotes
    let lote = validar_y_guardar_lote("/ruta/a/la/carpeta/de/entrada", carpeta_lotes)?;

    // Transformar los archivos del lote a PDF
    transformar_a_pdf(lote)?;

    // Eliminar los lotes antiguos que hayan pasado más de una semana desde su creación
    eliminar_lotes_antiguos(carpeta_lotes)?;

    Ok(())
}

fn validar_y_guardar_lote(carpeta_entrada: &str, carpeta_lotes: &Path) -> io::Result<String> {
    // Validar los archivos y guardar el lote en la carpeta de lotes
    let nombre_lote = "lote1"; // Aquí debes generar un nombre único para cada lote
    let ruta_lote = carpeta_lotes.join(nombre_lote);
    fs::create_dir(&ruta_lote)?;
    for archivo in fs::read_dir(carpeta_entrada)? {
        let archivo = archivo?;
        if archivo.path().is_file() {
            let ruta_archivo_destino = ruta_lote.join(archivo.file_name());
            fs::copy(archivo.path(), ruta_archivo_destino)?;
        }
    }
    Ok(nombre_lote.to_string())
}

fn transformar_a_pdf(lote: String) -> io::Result<()> {
    // Transformar los archivos del lote a PDF
    // ...
    Ok(())
}

fn eliminar_lotes_antiguos(carpeta_lotes: &Path) -> io::Result<()> {
    let una_semana_en_segundos = 604800;

    // Obtenemos la fecha actual
    let ahora = SystemTime::now();

    // Iteramos sobre los elementos de la carpeta "lotes"
    for entrada in fs::read_dir(carpeta_lotes)? {
        let entrada = entrada?;
        if !entrada.path().is_dir() {
            // Si la entrada no es una carpeta, la ignoramos
            continue;
        }

        // Obtenemos la fecha de creación de la carpeta del lote
        let metadatos = fs::metadata(entrada.path())?;
        let fecha_creacion = metadatos.created()?;

        // Calculamos la diferencia en segundos entre la fecha de creación y la fecha actual
        let diferencia_en_segundos = ahora.duration_since(fecha_creacion)?.as_secs();

        // Si ha pasado más de una semana, eliminamos la carpeta del lote
        if diferencia_en_segundos > una_semana_en_segundos {
            println!("Eliminando lote {:?}", entrada.path());
            fs::remove_dir_all(entrada.path())?;
        }
    }

    Ok(())
}
