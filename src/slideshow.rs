use reqwest::{self, Url};
use threadpool::ThreadPool;
use piston::window::{Size};
use opengl_graphics::*;
use image::DynamicImage;
use percent_encoding;
use serde_json;

use std;
use std::borrow::Borrow;
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::ffi::OsStr;
use image;

use flickr::{self, AccessToken, AuthenticatedClient, Photo};
use PretzboardError;

enum Dimension {
    Width(u32),
    Height(u32),
}

fn download_file(url: &Url, path: &Path) -> Result<(), PretzboardError> {
    let mut file = File::create(path)?;
    // TODO: Check that content type suggests it's actually an image
    // FIXME: reqwest::get creates a new client for each request. Ideally each thread would have its own client and that would be reused for each request that worker serviced
    reqwest::get(url.as_str())?.copy_to(&mut file)?;

    Ok(())
}

fn do_fetch_photo(url: &Url) -> Result<(), PretzboardError> {
    // let path = Path::new("photos");
    let percent_encoded_path = url.path();
    let cow = percent_encoding::percent_decode(percent_encoded_path.as_bytes()).decode_utf8()?;
    let path: &str = cow.borrow();
    let path = Path::new(path);
    let filename = path.file_name().ok_or_else(|| {
        PretzboardError::IoError(io::Error::new(
            io::ErrorKind::Other,
            "URL does not have file name",
        ))
    })?;

    // Check if photo has already been downloaded
    let mut storage_path = PathBuf::new();
    storage_path.push("photos");
    storage_path.push(filename);

    if storage_path.is_file() {
        println!("{} -> exists", url);
        Ok(())
    } else {
        // download the file
        println!("{} -> downloading", url);
        download_file(url, &storage_path)
    }
}

fn fetch_photo(photo: Photo, tx: std::sync::mpsc::Sender<Result<(), PretzboardError>>) {
    tx.send(do_fetch_photo(&photo.url_k))
        .expect("error sending to channel");
}

pub fn update_photostream(user_id: &str, client: &AuthenticatedClient) -> Result<(), PretzboardError> {
    // Request list of photos from Flickr
    // Download the ones that aren't in the cache
    // (optional) Clean up old images
    // Generate new JSON, move into place atomically

    let pool = ThreadPool::new(8);
    let (tx, rx) = channel();

    // Check the last 500 photos
    // TODO: photos page="2" pages="89" perpage="10" total="881">
    // Stop if there are fewer than 5 pages
    for page in 1..3 {
        let arguments = [
            ("min_taken_date", "1388494800".to_string()),
            ("content_type", "1".to_string()), // Photos only
            ("per_page", "100".to_string()),
            ("page", page.to_string()),
            ("extras", "url_k".to_string()),
        ];
        let photos = client.photos(user_id, &arguments)?;

        //println!("{:?}", photos);

        let photo_count = photos.len();

        for photo in photos {
            let tx = tx.clone();
            pool.execute(move || fetch_photo(photo, tx))
        }

        rx.iter().take(photo_count).for_each(|result| {
            if result.is_err() {
                println!("{:?}", result)
            }
        });
    }

    Ok(())
}

pub fn load_access_token<P: AsRef<Path>>(client: flickr::Client, path: P) -> Result<AuthenticatedClient, PretzboardError> {
    match File::open(path.as_ref()) {
        Ok(file) => {
            let access_token: AccessToken = serde_json::from_reader(file)?;
            Ok(AuthenticatedClient::new(client, access_token))
        }
        Err(e) => {
            println!("{:?}", e);
            let client = client.authenticate()?;

            // Save app data for using on the next run.
            let file = File::create(path.as_ref())?;
            let _ = serde_json::to_writer_pretty(file, client.access_token())?;

            Ok(client)
        }
    }
}

fn largest_dimension(size: Size) -> Dimension {
    if size.width > size.height {
        Dimension::Width(size.width)
    } else {
        Dimension::Height(size.height)
    }
}

pub fn zoom_for_image(window_size: Size, image_size: Size) -> f64 {
    match largest_dimension(image_size) {
        Dimension::Width(width) => window_size.width as f64 / width as f64,
        Dimension::Height(height) => window_size.height as f64 / height as f64,
    }
}

pub fn translation_for_image(window_width: u32, image_width: f64) -> f64 {
    (window_width as f64 / 2.) - (image_width / 2.)
}

pub fn load_photo<P: AsRef<Path>>(path: P) -> Result<Texture, PretzboardError> {
    println!("loading {:?}", path.as_ref());

    let photo = image::open(&path).map_err(|_err| {
        println!("{:?}", _err);
        PretzboardError::GraphicsError
    })?;

    let photo = match photo {
        DynamicImage::ImageRgba8(photo) => photo,
        x => x.to_rgba(),
    };

    Ok(Texture::from_image(&photo, &TextureSettings::new()))
}

pub fn available_photos(dir: &str) -> Result<Vec<PathBuf>, PretzboardError> {
    let mut photos = vec![];
    let jpg = OsStr::new("jpg");

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension()
            .map(|ext| ext == jpg && path.is_file())
            .unwrap_or(false)
        {
            println!("adding {:?}", path);
            photos.push(path);
        }
    }

    Ok(photos)
}

