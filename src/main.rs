use chrono::prelude::*;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process;

const RELEASE_URL: &str = "https://api.github.com/repos/ppy/osu/releases";
const USER_AGENT: &str = "OsuLinuxUpdater";
const FILE_NAME: &str = "osu.AppImage";
const DATA_DIR_NAME: &str = "osu-linux-launcher";
const ENV_DIR_NAME: &str = "OSU_DIR";

type GithubReleases = Vec<GithubReleaseReponse>;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Deserialize)]
struct GithubReleaseReponse {
    url: String,
    // id: u64,
    published_at: DateTime<Utc>,
    assets: Vec<GithubReleaseAsset>,
}

#[derive(Debug, Deserialize)]
struct GithubReleaseAsset {
    // id: u64,
    name: String,
    size: u64,
    browser_download_url: String,
}

fn get_lastest_release(client: &Client) -> Result<GithubReleaseReponse> {
    let res = client
        .get(RELEASE_URL)
        .send()?
        .json::<GithubReleases>()?
        .into_iter()
        .next()
        .ok_or("Cannot get osu release")?;

    Ok(res)
}

fn data_dir() -> Result<PathBuf> {
    env::var(ENV_DIR_NAME).map(PathBuf::from).or_else(|_| {
        let dir = dirs::data_dir()
            .ok_or("Cannot get the data directory")?
            .join(DATA_DIR_NAME);

        Ok(dir)
    })
}

fn main() -> Result<()> {
    let dir = data_dir()?;
    fs::create_dir(&dir).ok();

    println!("{} directory is {:?}", FILE_NAME, &dir);
    println!(
        "You can change it by set it an env variable {}",
        ENV_DIR_NAME
    );

    println!("Checking osu release");
    let client = Client::builder().user_agent(USER_AGENT).build()?;
    let release = get_lastest_release(&client)?;

    let osu_file = dir.join(FILE_NAME);
    let is_outdated = fs::File::open(&osu_file)
        .and_then(|file| file.metadata()?.modified())
        .ok()
        .map(|v| DateTime::<Utc>::from(v))
        .filter(|time| &release.published_at < time)
        .is_none();

    if is_outdated {
        println!("Downloading the newest osu version at {}", release.url);
        let asset = release
            .assets
            .iter()
            .find(|v| v.name == FILE_NAME)
            .ok_or("Cannot get the AppImage file")?;

        let data = client.get(&asset.browser_download_url).send()?.bytes()?;
        println!("Downloaded");
        println!("Replacing {:?} ({})", &osu_file, asset.size);
        fs::write(&osu_file, data)?;
        let permission = fs::Permissions::from_mode(0o777);
        fs::set_permissions(&osu_file, permission)?;
    }

    println!("Launching");
    process::Command::new(osu_file).output()?;
    println!("Bye");

    Ok(())
}
