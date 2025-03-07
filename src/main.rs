use actix_web::{
    body::BoxBody,
    error, get,
    http::header::ContentType,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Skolor {
    schools: HashMap<u32, HashMap<String, serde_json::Value>>,
    answers: HashMap<String, Vec<u32>>,
    sha1: HashMap<String, HashMap<String, serde_json::Value>>,
}

#[derive(Serialize)]
struct JsonObj {
    data: String,
}

// Responder
impl Responder for JsonObj {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .insert_header(("Cache-Control", "max-age=310"))
            .body(self.data)
    }
}

// This receives the queries
#[get("/entities")]
async fn index(req: HttpRequest) -> impl Responder {
    // A name which should not be there
    let params = match web::Query::<HashMap<String, String>>::from_query(req.query_string()) {
        Ok(data) => data,
        Err(_) => return Err(error::ErrorBadRequest("Missing params")),
    };
    let typed_data = match params.get("q") {
        Some(data) => data,
        None => return Err(error::ErrorInternalServerError("Server parameter q.")),
    };

    let data = match req.app_data::<Data<Mutex<Skolor>>>() {
        Some(d) => d,
        None => return Err(error::ErrorInternalServerError("Could not access mutex")),
    };
    let my_data = match data.lock() {
        Ok(d) => d,
        Err(_) => return Err(error::ErrorInternalServerError("Could not get the lock.")),
    };
    // Dummy query
    let names = match my_data.answers.get(typed_data) {
        Some(d) => d,
        None => {
            let body = "[]".to_string();
            return Ok(JsonObj { data: body });
        }
    };
    // This will be the result to send back as response in JSON
    let mut result: Vec<HashMap<String, serde_json::Value>> = Vec::new();
    // Now let us loop over
    for index in names {
        let a_school = my_data.schools.get(index).unwrap();
        result.push(a_school.clone());
    }
    // Return the result as JSON
    //
    let body = match serde_json::to_string(&result) {
        Ok(d) => d,
        Err(_) => return Err(error::ErrorInternalServerError("JSON error")),
    };
    return Ok(JsonObj { data: body });
}

#[get("/entities/{shafile}")]
async fn index_json(req: HttpRequest, path: web::Path<String>) -> Result<impl Responder> {
    let file_name = path.into_inner();

    let data = req.app_data::<Data<Mutex<Skolor>>>().unwrap();
    let my_data = data.lock().unwrap();
    // Dummy query
    let school = my_data.sha1.get(&file_name).unwrap();
    // Return the result as JSON
    Ok(web::Json(school.clone()))
}

#[get("/update")]
async fn update(req: HttpRequest) -> impl Responder {
    // First read the file
    // Please put it under tmpfs for fast reading.
    let file_data = fs::read_to_string("webdata.json").expect("Cound not read.");
    let new_skolor: Skolor = serde_json::from_str(&file_data).expect("JSON is not well formatted");
    let data = req
        .app_data::<Data<Mutex<Skolor>>>()
        .expect("Could not get lock to update");
    //.app_data::<Data<Mutex<HashMap<String, String>>>>()

    let mut my_data = data.lock().unwrap();
    my_data.schools = new_skolor.schools;
    my_data.answers = new_skolor.answers;
    my_data.sha1 = new_skolor.sha1;
    HttpResponse::Ok().body("updated")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //let data = Data::new(Mutex::new(HashMap::<String, String>::new()));
    let data = Data::new(Mutex::new(Skolor {
        schools: HashMap::new(),
        answers: HashMap::new(),
        sha1: HashMap::new(),
    }));
    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&data))
            .service(index)
            .service(index_json)
            .service(update)
        //.route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
