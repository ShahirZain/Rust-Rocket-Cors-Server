#![feature(proc_macro_hygiene,decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate lazy_static;
extern crate rocket_cors;
use std::sync::{Arc,Mutex};
use std::collections::HashMap;
use rocket_contrib::json::{Json,JsonValue};
use rocket::http::Method;
use rocket_cors::{
    AllowedHeaders,AllowedOrigins,Error,
    Cors,CorsOptions,
};
use rocket::State;

type ID = usize;


#[derive(Debug,PartialEq,Eq,Deserialize)]

struct Message{
    id:ID,
    contents:String
}

fn make_cors()->Cors{
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://best-ball.surge.sh",

    ]);
    CorsOptions{
        allowed_origins,
        allowed_methods: vec![Method::Get,Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building the cross")
}

#[get("/")]
fn hello() ->JsonValue{
    json!([
        {
            "id" : "01",
            "name": "mia"
        },
        {
            "id" : "02",
            "name": "khalifa"
        }
    ])
}

type MessageMap  = Mutex<HashMap<ID,String>>;

#[post("/add",data="<user_input>")]

fn helloPost(user_input:Json<Message>,map:State<'_,MessageMap>){
    print!("{:?}",user_input.0.contents);
}

fn rocket() -> rocket::Rocket{
    rocket::ignite()
    .mount("/",routes![hello,helloPost]).attach(make_cors())
    .manage(Mutex::new(HashMap::<ID,String>::new()))
}


fn main(){
    rocket().launch();
}