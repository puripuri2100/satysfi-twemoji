use std::fs::File;
use std::io::Write;

fn write_file(file_name: String, text: String) -> () {
  let mut file = File::create(file_name).unwrap();
  file.write_all(text.as_bytes()).unwrap();
}

pub mod unicode_point;
fn main () {
  let emoji_unicode_point_vec = unicode_point::make_unicode_point_vec ();
  let mut command_text = String::new();
  for v in emoji_unicode_point_vec.iter() {
    let mut file_name = String::new();
    let mut str_lst = String::new();
    for i in 0..v.len() {
      if i == 0 {
        file_name.push_str(v[i]);
      } else {
        let s = format!("-{}",v[i]);
        file_name.push_str(&s)
      };
      let s = format!("0x{};",v[i].to_uppercase());
      str_lst.push_str(&s)
    }
    let s = format!("  {}:\\test([{}]);\n",file_name,str_lst);
    command_text.push_str(&s);
  }
  let text = format!("
  @require: stdjabook

  @import: ../twemoji

  let-inline \\test lst =
    {{\\twemoji(string-unexplode lst);}}
  in
  document (|
    title = {{test}};
    author = {{\\@puripuri2100}};
    show-title = false;
    show-toc = false;
  |) '<
    +p{{ {} }}
  >",command_text);
  write_file("test/all.saty".to_string(), text);
}