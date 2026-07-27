use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use image::imageops::FilterType;
use image::{imageops, DynamicImage, ImageReader};
use slint::ComponentHandle;
use crate::Window;
use crate::AccountMenuState;
use crate::logger::error;
use crate::mojang::player::{fetch_raw, get_texture_value, TextureProfile, User};

pub async fn get_head(name_str: &str, size: u32) -> Result<PathBuf, Box<dyn Error + Send + Sync>> {

    let name: String = name_str.into();
    let user: User = fetch_raw(name.clone()).await?;
    let uuid = user.uuid.as_str();
    let root = dirs::home_dir().expect("Could not find home directory");
    let skins_path = root.join(".ferris/launcher/skins");
    fs::create_dir_all(&skins_path)?;

    // let final_file = skins_path.join(format!("{}/{}.png", uuid, uuid));println!("uuid = {:?}", uuid);

    let skin_path = format!("{}/skin.png", uuid);
    let final_skin_file = skins_path.join(&skin_path);
    fs::create_dir_all(&final_skin_file.parent().unwrap())?;
    let profile: TextureProfile = get_texture_value(String::from(name)).await?;
    let url = profile.textures.skin.url;
    let head_path = format!("{}/head_{}.png", uuid, size);
    let final_head_file = skins_path.join(&head_path);

    let download = if final_skin_file.exists() || final_head_file.exists() {

        let modified = fs::metadata(&final_skin_file)?.modified()?;
        SystemTime::now().duration_since(modified)? > Duration::from_secs(60 * 60 * 24)

    } else {

        true

    };

    if download {

        download_png(&url, final_skin_file.to_str().unwrap()).await?;

        fs::copy(&final_skin_file, &final_head_file)?;

        let head = ImageReader::open(&final_skin_file)?.decode()?;

        let shrunk = shrink_by_percent(size.clone() as f64, 12.0) as u32;

        let layer1 = head.crop_imm(8, 8, 8, 8);
        let layer1 = layer1.resize_exact(shrunk, shrunk, FilterType::Nearest);

        let layer2 = head.crop_imm(40, 8, 8, 8);
        let layer2 = layer2.resize_exact(size.clone(), size.clone(), FilterType::Nearest);

        let mut canvas = DynamicImage::new_rgba8(size.clone(), size.clone());

        let x = (&size - layer1.width()) / 2;
        let y = (&size - layer1.height()) / 2;

        imageops::overlay(&mut canvas, &layer1, x as i64, y as i64);
        imageops::overlay(&mut canvas, &layer2, 0, 0);

        canvas.save(&final_head_file)?;

    }

    Ok(PathBuf::from(final_head_file.as_path()))

}

async fn download_png(url: &str, path: &str) -> Result<(), Box<dyn Error + Send + Sync>> {

    let bytes = reqwest::get(url).await?.bytes().await?;
    fs::write(path, bytes)?;
    Ok(())

}

fn shrink_by_percent(value: f64, percent: f64) -> f64 {
    value * (1.0 - percent / 100.0)
}

pub async fn set_head(ui_weak: slint::Weak<Window>, name: &str) {

    let path = match get_head(name, 48).await {
        Ok(path) => path,
        Err(_) => return error(&format!("Could not find skin {}", name)).await,
    };

    let _ = ui_weak.upgrade_in_event_loop(move |ui| {
        if let Ok(image) = slint::Image::load_from_path(&path) {
            ui.global::<AccountMenuState>().set_head_image(image); 

        }
    });

}