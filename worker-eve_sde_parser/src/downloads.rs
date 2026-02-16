use std::fs;
use std::process::Command;

use crate::{Error, FOLDER_INPUT, Result};

pub async fn checksum(
    directory: &str,
) -> Result<String> {
    download_file(
        directory,
        "https://eve-static-data-export.s3-eu-west-1.amazonaws.com/tranquility/checksum",
        "checksum"
    )
    .await?;

    let path = format!("{directory}/{FOLDER_INPUT}/checksum");
    let checksum = fs::read_to_string(path.clone()).map_err(|e| Error::FileWriteError(e, path))?;
    let sde_checksum: String = checksum
        .split("\n")
        .find(|x| x.ends_with("sde.zip"))
        .map(|x| x.split(" "))
        .unwrap()
        .collect::<Vec<_>>()
        .first()
        .map(|x| x.to_string())
        .unwrap_or_default();

    Ok(sde_checksum)
}

pub async fn download_assets(
    directory: &str,
) -> Result<()> {
    let _ = fs::create_dir(format!("{directory}/{FOLDER_INPUT}"));

    download_file(
        directory,
        "https://developers.eveonline.com/static-data/tranquility/eve-online-static-data-3201939-yaml.zip",
        "sde.zip",
    )
    .await?;

    download_file(
        directory,
        "https://sde.hoboleaks.space/tq/industrymodifiersources.json",
        "industrymodifiersources.json",
    )
    .await?;

    download_file(
        directory,
        "https://sde.hoboleaks.space/tq/industrytargetfilters.json",
        "industrytargetfilters.json",
    )
    .await?;

    download_file(
        directory,
        "https://sde.hoboleaks.space/tq/repackagedvolumes.json",
        "repackagedvolumes.json",
    )
    .await?;

    unzip(
        directory,
        "sde.zip",
    )?;

    Ok(())
}

async fn download_file(
    directory: &str,
    url:       &str,
    filename:  &str,
) -> Result<()> {
    let out = format!("{directory}/{FOLDER_INPUT}/{filename}");
    let _ = fs::remove_file(&out);

    let sde = reqwest::get(url)
        .await?
        .bytes()
        .await?;
    fs::write(&out, sde).map_err(|e| Error::FileWriteError(e, out))?;

    Ok(())
}

fn unzip(
    directory: &str,
    filename:  &str,
) -> Result<()> {
    let input = format!("{directory}/{FOLDER_INPUT}/{filename}");
    let out = format!("{directory}/{FOLDER_INPUT}");

    Command::new("unzip")
        .args([input, "-d".into(), out])
        .output()
        .map(drop)
        .map_err(Error::UnzipError)
}
