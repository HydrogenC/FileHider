use std::env::args;
use std::io::stdin;
use std::fs;
use std::path::Path;
use std::io::Write;
use std::os::windows::ffi::OsStrExt;

extern crate winapi;

use winapi::um::winnt::*;
use winapi::um::fileapi::*;
use std::ffi::OsStr;

#[cfg(windows)]
fn main() {
    let mut arg = args();
    let mut vc: Vec<String> = vec![];
    let cfg = Path::new("rl.cfg");

    let get_cfg_full_path = || {
        let mut tmp: Vec<u16> = if cfg.is_absolute() {
            cfg.as_os_str().encode_wide().collect()
        } else {
            std::env::current_dir().unwrap().join(cfg).as_os_str().encode_wide().collect()
        };
        tmp.push(0);
        tmp
    };

    if cfg.exists() {
        unsafe {
            SetFileAttributesW(get_cfg_full_path().as_ptr(), FILE_ATTRIBUTE_NORMAL);
            for i in fs::read_to_string(cfg).unwrap_or_default().split(|c| c == '\n' || c == '\r') {
                if i != "" && Path::new(i).exists() {
                    vc.push(i.to_string());
                }
            }
        }
    }

    match arg.len() {
        1 => unsafe {
            let mut tmp = String::new();
            println!("1. 隐藏指定文件");
            println!("2. 显示指定文件");
            println!("3. 清空文件列表");
            stdin().read_line(&mut tmp);
            tmp = tmp.trim_end().to_string();

            match tmp.as_str() {
                "1" => {
                    for i in &vc {
                        let mut str: Vec<u16> = OsStr::new(i).encode_wide().collect();
                        str.push(0);
                        SetFileAttributesW(str.as_ptr(), FILE_ATTRIBUTE_HIDDEN | FILE_ATTRIBUTE_SYSTEM);
                    }
                }
                "2" => {
                    for i in &vc {
                        let mut str: Vec<u16> = OsStr::new(i).encode_wide().collect();
                        str.push(0);
                        SetFileAttributesW(str.as_ptr(), FILE_ATTRIBUTE_NORMAL);
                    }
                }
                "3" => {
                    if cfg.exists() {
                        fs::remove_file(cfg);
                    }
                }
                _ => ()
            }
        }
        2 => {
            let to_add = arg.nth(1).unwrap().to_string();
            if !vc.contains(&to_add) {
                vc.push(to_add);
            }

            let mut fl = fs::File::create(cfg).unwrap();
            for mut i in vc {
                i += "\r\n";
                fl.write_all(i.as_bytes());
            }
        }
        _ => ()
    }
    unsafe {
        SetFileAttributesW(get_cfg_full_path().as_ptr(), FILE_ATTRIBUTE_HIDDEN | FILE_ATTRIBUTE_SYSTEM);
    }
}
