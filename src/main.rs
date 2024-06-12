use chrono::{DateTime, NaiveDate};
use reqwest::get;
use serde_json::Value;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    println!("Consultando en la página https://datosabiertos.bogota.gov.co/dataset/racionamiento-agua-bogota-d-c");

    let fechas_actualizadas = get(r#"https://services1.arcgis.com/J5ltM0ovtzXUbp7B/ArcGIS/rest/services/EsquemaRestriccion/FeatureServer/0/query?returnGeometry=true&where=1=1&outSr=4326&outFields=*&inSr=4326&geometry={"xmin":-74.53125,"ymin":4.214943141390651,"xmax":-73.125,"ymax":5.61598581915534,"spatialReference":{"wkid":4326}}&geometryType=esriGeometryEnvelope&spatialRel=esriSpatialRelIntersects&geometryPrecision=6&f=geojson"#)
        .await?
        .text()
        .await?;

    println!("Listo \n======================================================= ");

    let fechas_actualizadas: Value = serde_json::from_str(&fechas_actualizadas).unwrap();
    let fechas_actualizadas: &Value = fechas_actualizadas.get("features").unwrap();
    let fechas_actualizadas: &Vec<Value> = fechas_actualizadas.as_array().unwrap();

    let mut localidades: Vec<Vec<&str>> = vec![];
    let mut fechas: Vec<NaiveDate> = vec![];
    for valor in fechas_actualizadas {
        let mut vec_localidades: Vec<Vec<&str>> = vec![valor.as_object().unwrap()["properties"].as_object().unwrap()["LOCALIDADE"].as_str().unwrap().split(", ").collect()];

        localidades.append(&mut vec_localidades);

        let mut vec_fechas = vec![de_numero_a_fecha(valor.as_object().unwrap()["properties"].as_object().unwrap()["FECHA_INI"].as_i64().unwrap_or(0))];

        fechas.append(&mut vec_fechas);

    };
    let mut contador = 1;
    println!("Seleccione un conjunto donde esté la localidad a consultar: \n");
    for i in localidades {
        if contador > 8 {
            break
        }
        println!("{contador}. {:?}", i);
        contador += 1;
    }
    
    let seleccion = loop {
        let mut seleccion = String::new();

        io::stdin()
            .read_line(&mut seleccion)
            .expect("Fallo al leer la línea");

        let seleccion: usize = match seleccion
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => continue
            };

        if seleccion > 8 {
            println!("Sólo hay 8 opciones!");
            continue;
        } else {
            break seleccion - 1;
        }
    };
    
    println!("La fecha de corte de agua en tu localidad es: {:?}", fechas[seleccion]);
    Ok(())
}

fn de_numero_a_fecha(numero: i64) -> NaiveDate {
    let fecha = DateTime::from_timestamp_millis(numero).expect("invalid timestamp").to_utc().date_naive();
    return fecha;
}