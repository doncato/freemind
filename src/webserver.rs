pub mod handler {
    use actix_web::{
        http::header, middleware::Logger, route, web, App, HttpRequest, HttpResponse, HttpServer,
        Result,
    };

    #[route(
        r"/api/v1/{id:.*[ A-Za-z0-9_.%+-]}/get/speech/{directive:.*}",
        method = "GET",
        method = "HEAD"
    )]
    async fn get_speech(req: HttpRequest, state: web::Data<AppState>) -> Result<HttpResponse> {
        let auth = req.match_info().query("id");
        let directive = req.match_info().query("directive");
        let user = match Database::init(&state.db_path).unwrap().get_user(auth) {
            Ok(val) => val,
            Err(err) => {
                log::warn!("An error occured:\n {}", err);
                return Ok(HttpResponse::InternalServerError().body("500 - InternalServerError"));
            }
        };
        if user.is_none() {
            return Ok(HttpResponse::Unauthorized().body("401 - Unauthorized"));
        }
        // At this point the request is valid
        let raw_speech = match directive {
            "generic.mp3" => speech::get_generic(user.unwrap(), &state.tts_api_key).await, // Generic information, time, date
            "weather.mp3" => speech::get_weather(user.unwrap(), &state.tts_api_key).await, // Information about the weather
            "calendar.mp3" => speech::get_calendar(user.unwrap(), &state.tts_api_key).await, // Information about the calendar
            text => speech::get_whatever(text, &state.tts_api_key).await, // Testing
                                                                          //_ => return Ok(HttpResponse::NotFound().body("404 - Not Found")),
        };

        let response = match raw_speech {
            Ok(val) => val,
            Err(err) => {
                log::warn!("An error occured:\n {:?}", err);
                return Ok(HttpResponse::InternalServerError().body("500 - InternalServerError"));
            }
        };

        return Ok(HttpResponse::Ok()
            .insert_header(("content-type", "audio/mpeg"))
            .body(response));
    }

    #[actix_web::main]
    pub async fn run(state: AppState) -> std::io::Result<()> {
        let port = state.port;
        let workers = 4;
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(state.clone()))
                .service(get_speech)
        })
        .workers(workers)
        .bind(("0.0.0.0", port))?
        .run()
        .await
    }
}
