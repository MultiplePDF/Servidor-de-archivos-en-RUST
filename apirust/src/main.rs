#[macro_use] extern crate rocket;

use base64::decode;
use json::{Array, JsonValue};
use std::fs::File;
use rocket::{serde::{json::{Json, serde_json}, DeserializeOwned}, Data};
use std::io::Write;
use serde_json::{Value};
use std::fs;
use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};
use std::io::BufWriter;
use zip::write::FileOptions;
use zip::result::ZipResult;
use zip::ZipWriter;
use std::io;
use std::io::{ BufReader, Read};

#[derive(serde:: Deserialize, Debug)]
struct Message {
   subBatchID: String ,
   userID: String,
   files: Vec<serde_json::Value>,
}

#[derive(serde::Deserialize, Debug)]
struct InnerJSON {
    base64: String,
    checksum: String,
    fileID: i64,
    fileName: String,
    size: u32,
    subBatchID: String,
    url: String,
}

#[get("/")]
fn index() -> String {
    "Hola mundo!".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, decode_base64,download_file,download_batch])
}



#[get("/download/<user>/<batch>/<filename>")]
async fn download_file(user: &str, batch: &str, filename: &str) -> Option<NamedFile> {
    let file_path = format!("/opt/services/servidor/{}/{}/{}", user, batch, filename);
    NamedFile::open(file_path).await.ok()
}



#[get("/download_batch/<user>/<batch>")]
async fn download_batch(user: &str, batch: &str) -> io::Result<NamedFile> {
    let dir_path = format!("/opt/services/servidor/{}/{}", user, batch);
    let zip_file_path = format!("/opt/services/servidor/{}/{}.zip", user, batch);

    // create a zip file to write the batch files
    let zip_file = File::create(&zip_file_path)?;
    let mut zip_writer = zip::ZipWriter::new(zip_file);

    // Add all files in the batch directory to the zip file
    let dir = Path::new(&dir_path);
    let files = fs::read_dir(dir)?;
    for file in files {
        let file_path = file?.path();
        let file_name = file_path.file_name().unwrap().to_string_lossy().to_string();
        let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);
        zip_writer.start_file(&file_name, options);
        let mut file = BufReader::new(File::open(file_path)?);
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        zip_writer.write_all(&buffer)?;
    }

    // Finish writing the zip file
    zip_writer.finish()?;

    // Return the NamedFile for the zip file
    Ok(NamedFile::open(zip_file_path).await?)
}



#[post("/decode", format = "json", data = "<message>")]
fn decode_base64(message:String) -> Result<String, String> {
    //println!("{}",&message);

    let mut object: Message = serde_json::from_str(&message).unwrap();

    let dir_user = format!("/opt/services/servidor/p{}", object.userID);

    if !Path::new(&dir_user).exists() {
        match fs::create_dir(&dir_user) {
            Ok(_) => println!("Se ha creado la carpeta del usuario: {}", dir_user),
            Err(err) =>  return Err(format!("Error al crear la carpeta del usuario: {}", err)),
        };
    }

    let dir_name = format!("{}/{}",dir_user, object.subBatchID);

    if !Path::new(&dir_name).exists() {
        match fs::create_dir(&dir_name) {
            Ok(_) => println!("Se ha creado la carpeta del lote: {}", dir_name),
            Err(err) =>  return Err(format!("Error al crear la carpeta del lote: {}", err)),
        };
    }

    for x in object.files {
        let object_value: Value = serde_json::from_str(&x.to_string()).unwrap();
        let my_json: InnerJSON = serde_json::from_value(object_value).unwrap();

        let decoded_bytes = match decode(my_json.base64) {
            Ok(bytes) => bytes,
           Err(err) => return Err(format!("Error al decodificar: {}", err)),
        };
        let path = format!("{}/{}", dir_name, my_json.fileName);
        let mut file = match File::create(path) {
            Ok(file) => file,
            Err(err) => return Err(format!("Error al crear el archivo: {}", err)),
        };
        file.write_all(&decoded_bytes);

    }
   Ok("Exito".to_string())
}

