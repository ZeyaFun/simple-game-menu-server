extern crate image_base64;
use std::{error::Error, fs};
use uuid::Uuid;

pub async fn save_image_by_base64(base64: String) -> Result<String, Box<dyn Error>> {
    let base_path: String = "./".to_string();
    let image = image_base64::from_base64(base64);
    let file_name = Uuid::new_v4().to_string() + ".jpg";
    let path = base_path + &file_name;
    println!("{}", path.clone());
    match fs::write(path.clone(), image) {
        Ok(()) => (),
        Err(e) => panic!("存储时发生了错误！{:?}", e),
    };
    Ok(path)
}

pub async fn update_image_by_base64(
    path: String,
    base64: String,
) -> Result<String, Box<dyn Error>> {
    let image = image_base64::from_base64(base64);
    println!("{}", path.clone());
    match fs::write(path.clone(), image) {
        Ok(()) => (),
        Err(e) => panic!("更新时发生了错误！{:?}", e),
    };
    Ok(path)
}
pub async fn load_image_to_base64(image_path: String) -> Result<String, Box<dyn Error>> {
    let base64 = image_base64::to_base64(&image_path);
    Ok(base64)
}

pub async fn delete_image(image_path: String) -> Result<(), Box<dyn Error>> {
    fs::remove_file(image_path).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    Ok(())
}
/*
the size for values of type `dyn StdError` cannot be known at compilation time
fn main() {
  let base64 = "base64 String";
  let image = image_base64::from_base64(base64);

  let image_path = "local image file path"
  let base64 = image_base64::to_base64(image_path);
}
*/
