extern crate iron;
#[macro_use] extern crate mime;

use iron::prelude::*;
use iron::status;

fn main() {
    println!("Serwer dostępny pod adresem localhost:3000...");
    Iron::new(get_form).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(r#"
        <title> Semiconductors simulator</title>
        <form action="/sim" method = "post">
            <input type = "text" name = "n"/>
            <button type="submit">Simulate</button>
        </form>
    "#);
    Ok(response)
}














