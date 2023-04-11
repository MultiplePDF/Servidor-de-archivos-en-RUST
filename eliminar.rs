use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;
use std::time::{Duration, SystemTime};

fn eliminar_lotes_antiguos() -> io::Result<()> {
    let una_semana_en_segundos = 604800;

    // Obtenemos la fecha actual
    let ahora = SystemTime::now();

    // Iteramos sobre los elementos de la carpeta "lotes"
    let carpeta_lotes = Path::new("ruta_de_la_carpeta_de_lotes");
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
