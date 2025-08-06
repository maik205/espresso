use espresso::Espresso;
use request::EspressoRequest;
use response::EspressoResponse;

mod espresso;
mod threads;
mod response;
mod error;
mod request;
fn main() {
    let mut app = Espresso::new("localhost:3200");
    app.all("/world", |req: &EspressoRequest, res: &mut EspressoResponse| {
        println!("hello?");
        res.send("Hello world!");
        res.send(
            &format!("I saw {} in my dreams", req.body.clone().unwrap_or("Nothing!".to_string()))
        );
    });
    app.all("/test", |req: &EspressoRequest, res: &mut EspressoResponse| {
        res.send("Woohoo!\n");
    });
    app.listen();
}

#[cfg(test)]
mod tests;
