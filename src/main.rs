use actix_web::{get, web, App, HttpServer};

struct AppState {
    app_name: String, //structure containing a String app_name
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String { // asynchronous function index
    let app_name = &data.app_name; // pulls in app name, borrows from argument
    format!("Hello {app_name}!") // returns it in reformatted
}

#[actix_web::main]
async fn main() -> std::io::Result<()> { //main function which returns a Result enum
    HttpServer::new(|| { //startingup a httpserver
        App::new() //new application, where:
            .app_data(web::Data::new(AppState { //set app data to a new AppState
                app_name: String::from("Actix Web"), // where the item appname is set to "Actix
                                                     // Web"
            }))
            .service(index) // call index, which returns the app reformatted.
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
