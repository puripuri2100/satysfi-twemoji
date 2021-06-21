use std::process::Command;
use std::fs::File;
use std::io::Write;

fn write_file(file_name: String, text: String) -> () {
  let mut file = File::create(file_name).unwrap();
  file.write_all(text.as_bytes()).unwrap();
}

pub mod unicode_point;
fn main() {
  let satyh_path = "emoji_images/twemoji_satyh/";
  let images_path = "emoji_images/";
  let emoji_unicode_point_vec = unicode_point::make_unicode_point_vec();
  let mut file_name_vec = vec![];
  for v in emoji_unicode_point_vec.iter() {
    let mut file_name = String::new();
    for i in 0..v.len() {
      if i == 0 {
        file_name.push_str(v[i])
      } else {
        let s = format!("-{}", v[i]);
        file_name.push_str(&s)
      }
    }
    file_name_vec.push(file_name);
  }
  for file_name in file_name_vec.iter() {
    let com = format!(
      "svg2saty -i emoji_images/twemoji_svg/{}.svg -o {}{}.satyh -n twemoji-{}",
      file_name, satyh_path, file_name, file_name
    );
    let _ = Command::new("sh")
      .arg("-c")
      .arg(com)
      .output()
      .expect("failed to execute process");
  }
  let mut concat_file_name_str = String::new();
  for i in 0..file_name_vec.len() {
    let file_name = &file_name_vec[i];
    let s = format!(" {}{}.satyh", satyh_path, file_name);
    concat_file_name_str.push_str(&s)
  }
  write_file("emoji_images/twemoji_graphics.satyh".to_string(), concat_file_name_str);
  let mut satysfi_list_str_1500 = String::new();
  for i in 0..1500 {
    let file_name = &file_name_vec[i];
    let s = format!("(`{}`,twemoji-{});", file_name, file_name);
    satysfi_list_str_1500.push_str(&s)
  }
  let mut satysfi_list_str_max = String::new();
  for i in 1500..file_name_vec.len() {
    let file_name = &file_name_vec[i];
    let s = format!("(`{}`,twemoji-{});", file_name, file_name);
    satysfi_list_str_max.push_str(&s)
  }
  let _ = Command::new("sh")
    .arg("-c")
    .arg(format!(
      "sed -i -e '/@require:/d' {}twemoji_graphics.satyh",
      images_path
    ))
    .output()
    .expect("failed to execute process");
  let _ = Command::new("sh")
    .arg("-c")
    .arg(format!(
      "sed -i -e \"1i @import: ../../svg2saty/satysfi/svg\" {}twemoji_graphics.satyh",
      images_path
    ))
    .output()
    .expect("failed to execute process");
  let _ = Command::new("sh")
    .arg("-c")
    .arg(format!(
      "sed -i -e '$a let twemoji-list = [{}' {}twemoji_graphics.satyh",
      satysfi_list_str_1500, images_path
    ))
    .output()
    .expect("failed to execute process");
  let _ = Command::new("sh")
    .arg("-c")
    .arg(format!(
      "sed -i -e '$a {}]' {}twemoji_graphics.satyh",
      satysfi_list_str_max, images_path
    ))
    .output()
    .expect("failed to execute process");
}
