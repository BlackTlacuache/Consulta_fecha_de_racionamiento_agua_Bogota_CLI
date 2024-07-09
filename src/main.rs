use chrono::{DateTime, Datelike, NaiveDate};
use reqwest::get;
use serde_json::Value;
use std::{borrow::Borrow, io};
use indexmap::IndexMap as HashMap;

#[derive(Debug)]
struct Sector {
    localidad: String,
    fecha: NaiveDate,
    delimitacion: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Consultando en la página https://datosabiertos.bogota.gov.co/dataset/racionamiento-agua-bogota-d-c");

    let fechas_actualizadas = obtener_datos().unwrap();
    println!("Listo \n======================================================= ");

    let fechas_actualizadas: Value = serde_json::from_str(&fechas_actualizadas).unwrap();
    let fechas_actualizadas: &Value = fechas_actualizadas.get("features").unwrap();
    let fechas_actualizadas = fechas_actualizadas.as_array().unwrap().into_iter();

    let mut datos: HashMap<String, Sector> = HashMap::new();
    let mut cont = 1;
    for valor in fechas_actualizadas {
        let vec_localidades: Vec<&str> = valor.as_object().unwrap()["properties"]
            .as_object()
            .unwrap()["LOCALIDADE"]
            .as_str()
            .unwrap()
            .split(", ")
            .collect();

        let fecha = de_numero_a_fecha(
            valor.as_object().unwrap()["properties"]
                .as_object()
                .unwrap()["FECHA_INI"]
                .as_i64()
                .unwrap_or(0),
        );

        let sector = valor.as_object().unwrap()["properties"]
        .as_object()
        .unwrap()["SECTOR"]
        .as_str()
        .unwrap();

        
        for localidad in vec_localidades {
            let loc =  cont.to_string();

            let junte = Sector{
                localidad: localidad.to_string(),
                fecha: fecha,
                delimitacion: sector.to_string(),
            };
            datos.insert(loc, junte);
            cont += 1;
        }
    }
    let caracteres_a_eliminar = ['"'];
    let opcion = loop {
        for (sector, junte) in datos.borrow() {
            // Quitamos las comillas de cada palabra
            let localidad: String = junte.localidad.chars()
            .filter(|c| !caracteres_a_eliminar.contains(c))
            .collect();

            println!("sector #{sector} {localidad}");
        }
        println!("\nSeleccione un sector con un número entre 1 y {:?}:", datos.len());
        let mut opcion = String::new();

        io::stdin()
            .read_line(&mut opcion)
            .expect("Fallo al leer la línea");

        let opcion: u32 = match opcion
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => continue
            };
        if opcion < datos.len() as u32 + 1 {
            break opcion;
        }
    };
    
    let desempaque = datos.get(&opcion.to_string()).unwrap();
    let dias = ["lunes", "martes", "miercoles", "jueves", "viernes", "sábado", "domingo"];
    let meses = ["Enero", "Febrero", "Marzo","Abril","Mayo", "Junio", "Julio", "Agosto", "Septiembre", "Octubre", "Noviembre", "Diciembre"];
    println!("El próximo racionamiento en la localidad de {} es el {} {} de {} \n\nLa delimitación exacta es:\n\n--> {}", 
        desempaque.localidad, 
        dias[desempaque.fecha.weekday().num_days_from_monday() as usize], 
        desempaque.fecha.day(),
        meses[desempaque.fecha.month0() as usize],
        desempaque.delimitacion
    );
    
    Ok(())
}

#[tokio::main]
async fn obtener_datos() -> Result<String, Box<dyn std::error::Error>> {
    let fechas_actualizadas = get(r#"https://services1.arcgis.com/J5ltM0ovtzXUbp7B/ArcGIS/rest/services/EsquemaRestriccion/FeatureServer/0/query?returnGeometry=true&where=1=1&outSr=4326&outFields=*&inSr=4326&geometry={"xmin":-74.53125,"ymin":4.214943141390651,"xmax":-73.125,"ymax":5.61598581915534,"spatialReference":{"wkid":4326}}&geometryType=esriGeometryEnvelope&spatialRel=esriSpatialRelIntersects&geometryPrecision=6&f=geojson"#)
        .await?
        .text()
        .await?;
    return  Ok(fechas_actualizadas);
}

fn de_numero_a_fecha(numero: i64) -> NaiveDate {
    let fecha = DateTime::from_timestamp_millis(numero)
        .expect("invalid timestamp")
        .to_utc()
        .date_naive();
    return fecha;
}
