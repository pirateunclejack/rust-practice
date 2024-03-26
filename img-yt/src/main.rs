use error_chain::error_chain;
use std::fs::File;
use std::io::copy;
use std::thread::sleep;
use std::time::Duration;
use tempfile::Builder;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let tmp_dir = Builder::new().prefix("example").tempdir()?;
    let target = "https://www.rust-lang.org/static/images/rust-logo-blk.svg";
    let response = reqwest::get(target).await?;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("file to download: '{}'", fname);

        let fname = tmp_dir.path().join(fname);
        println!("will be located uder: {:#?}", fname);

        File::create(fname)?
    };

    let content = response.text().await?;

    copy(&mut content.as_bytes(), &mut dest)?;
    sleep(Duration::from_secs(200)); // wait for file to be written to disk

    Ok(())
}
