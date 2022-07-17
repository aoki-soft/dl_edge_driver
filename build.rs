extern crate winapi;
extern crate winres;

fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icon.ico");
        res.set_language(winapi::um::winnt::MAKELANGID(
            winapi::um::winnt::LANG_JAPANESE, 
             winapi::um::winnt::SUBLANG_JAPANESE_JAPAN));
        res.compile().unwrap();
    }
  }
  