use crate::utils::errors::DictionaryError;
use reqwest::blocking::get;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Download the raw dictionary from the given URL if it doesn't already exist.
/// This function either returns Ok when the dictionary is downloaded or a DictionaryError
pub fn download_raw_dictionary(url: &str, destination: &Path) -> Result<(), DictionaryError> {
    println!("Downloading raw dictionary from {}...", url);
    let response = get(url)
        .map_err(|_| DictionaryError::DownloadError)?
        .text()
        .map_err(|_| DictionaryError::DownloadError)?;

    let mut file = File::create(destination).map_err(|_| DictionaryError::PathError)?;
    file.write_all(response.as_bytes())
        .map_err(|_| DictionaryError::PathError)?;

    println!("Raw dictionary downloaded and saved to {:?}", destination);
    Ok(())
}
