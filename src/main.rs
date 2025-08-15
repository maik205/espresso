use espresso::{espresso::Espresso, request::EspressoRequest, response::EspressoResponse};

fn main() {
    let mut app = Espresso::new("localhost:3200");
    app.all("/", |req: &EspressoRequest, res: &mut EspressoResponse| {
        res.send("Hello world!");
        res.send(&format!(
            "I saw {} in my dreams",
            req.body.clone().unwrap_or("Nothing!".to_string())
        ));
    });
    app.all(
        "/test",
        |_req: &EspressoRequest, res: &mut EspressoResponse| {
            res.send("Woohoo!\n");
        },
    );
    app.listen();
}

#[cfg(test)]
mod tests;
