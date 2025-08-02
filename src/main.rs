use espresso::Espresso;
use request::EspressoRequest;
use response::EspressoResponse;

mod espresso;
mod threads;
mod response;
mod error;
mod request;
fn main() {
    let mut app = Espresso::new("localhost:4200");
    app.all("/", |req: &EspressoRequest, res: &mut EspressoResponse| {
        res.send("Hello world!");
        res.send(
            &format!("I saw {} in my dreams", req.body.clone().unwrap_or("Nothing!".to_string()))
        );
    });
    app.listen();
}

#[cfg(test)]
mod tests;
