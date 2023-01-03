use tide::Request;

async fn can_i_kick_it(req: Request<()>) -> tide::Result<String> {
    let item = req.param("it")?;
    let val = kickable::validate(item);
    let res = format!("{val}");
    Ok(res)
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let mut app = tide::new();
    app.at("/:it").get(can_i_kick_it);
    app.listen("0.0.0.0:31337").await?;
    Ok(())
}
