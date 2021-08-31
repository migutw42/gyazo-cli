use gyazo_cli::gyazo_result::GyazoResult;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let filename_list: &[String] = &args[1..];

    for filepath in filename_list.into_iter() {
        if let Ok(url) = upload_image(filepath) {
            println!("{}", url);
        }
    }
}

fn upload_image(filepath: &String) -> Result<String, Box<dyn std::error::Error>> {
    let access_token = get_access_token();

    let form = reqwest::blocking::multipart::Form::new()
        .text("access_token", access_token)
        .file("imagedata", filepath)?;

    let client = reqwest::blocking::Client::new();
    let resp = client
        .post("https://upload.gyazo.com/api/upload")
        .multipart(form)
        .send()?;

    let gyazo_result: GyazoResult = resp.json()?;

    Ok(gyazo_result.url)
}

fn get_access_token() -> String {
    match std::env::var("GYAZO_ACCESS_TOKEN") {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1)
        }
    }
}
