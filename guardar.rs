use std::fs::{create_dir_all, File};
use std::io::{Error, Result};
use std::path::{Path, PathBuf};

// Función para guardar los archivos PDF en el servidor de archivos
fn guardar_archivos_pdf(usuario: &str, lote: &str, archivos: &[&str]) -> Result<()> {
    // Creamos la carpeta del usuario y la carpeta del lote dentro de ella
    let path_usuario = Path::new("ruta_del_servidor").join(usuario);
    let path_lote = path_usuario.join(lote);
    create_dir_all(path_lote.clone())?;

    // Guardamos cada archivo PDF en la carpeta del lote
    for archivo in archivos {
        let nombre_archivo = PathBuf::from(archivo).file_name().unwrap();
        let path_archivo = path_lote.join(nombre_archivo).with_extension("pdf");
        let mut archivo_pdf = File::create(path_archivo)?;
        // Aquí se escribe el contenido del archivo PDF en el archivo creado
        // No incluyo esa parte ya que depende de cómo hayas transformado los archivos a PDF
    }
    Ok(())
}
