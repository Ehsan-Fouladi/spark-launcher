use crate::prelude::*;

#[derive(Clone)]
pub struct SearchEntry {
    pub title: String,
    pub subtitle: String,
    pub category: String,
    pub action: ActionType,
    pub icon: Option<Image>,
}

pub fn index_system_items() -> Vec<SearchEntry> {
    let mut items: Vec<SearchEntry> = Vec::new();

    let sys_commands = vec![
        (
            "Lock Screen",
            "Window Locked",
            "rundll32.exe user32.dll,LockWorkStation",
        ),
        ("Open Terminal", "Windows Terminal", "wt"),
        ("Task Manager", "Management Windows Task", "taskmgr"),
        ("System Settings", "Setting", "ms-settings:"),
    ];

    for (title, sub, cmd) in sys_commands {
        items.push(SearchEntry {
            title: title.to_string(),
            subtitle: sub.to_string(),
            category: "SYS".to_string(),
            action: ActionType::SystemCmd(cmd),
            icon: None,
        });
    }

    let mut paths = vec![PathBuf::from(
        r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs",
    )];
    if let Ok(appdata) = std::env::var("APPDATA") {
        paths.push(PathBuf::from(appdata).join(r"Microsoft\Windows\Start Menu\Programs"))
    }

    for base_path in paths {
        for entry in WalkDir::new(base_path).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("lnk") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    items.push(SearchEntry {
                        title: stem.to_string(),
                        subtitle: path.to_string_lossy().to_string(),
                        category: "APP".to_string(),
                        action: ActionType::OpenPath(path.to_path_buf()),
                        icon: extract_icon(path),
                    })
                }
            }
        }
    }

    items
}

pub fn extract_icon(path: &Path) -> Option<Image> {
    use std::os::windows::ffi::OsStrExt;
    use windows::Win32::Foundation::SIZE;
    use windows::Win32::Graphics::Gdi::{
        BI_RGB, BITMAP, BITMAPINFO, BITMAPINFOHEADER, CreateCompatibleDC, DIB_RGB_COLORS, DeleteDC,
        DeleteObject, GetDIBits, GetObjectW, SelectObject,
    };
    use windows::Win32::System::Com::{COINIT_APARTMENTTHREADED, CoInitializeEx, CoUninitialize};
    use windows::Win32::UI::Shell::{
        IShellItemImageFactory, SHCreateItemFromParsingName, SIIGBF_ICONONLY,
    };
    use windows::core::PCWSTR;

    let wide_path: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();

    unsafe {
        let co_init = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

        let factory: Result<IShellItemImageFactory, _> =
            SHCreateItemFromParsingName(PCWSTR(wide_path.as_ptr()), None);

        let image_result =
            factory.and_then(|f| f.GetImage(SIZE { cx: 48, cy: 48 }, SIIGBF_ICONONLY));

        let hbitmap = match image_result {
            Ok(bmp) => bmp,
            Err(_) => {
                if co_init.is_ok() {
                    CoUninitialize();
                }
                return None;
            }
        };

        let mut bmp_info = BITMAP::default();
        GetObjectW(
            hbitmap.into(),
            std::mem::size_of::<BITMAP>() as i32,
            Some(&mut bmp_info as *mut _ as *mut _),
        );

        let width = bmp_info.bmWidth as u32;
        let height = bmp_info.bmHeight as u32;

        if width == 0 || height == 0 {
            let _ = DeleteObject(hbitmap.into());
            if co_init.is_ok() {
                CoUninitialize();
            }
            return None;
        }

        let hdc = CreateCompatibleDC(None);
        let old_bmp = SelectObject(hdc, hbitmap.into());

        let mut bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: width as i32,
                biHeight: -(height as i32),
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut bgra_pixels = vec![0u8; (width * height * 4) as usize];
        GetDIBits(
            hdc,
            hbitmap,
            0,
            height,
            Some(bgra_pixels.as_mut_ptr() as *mut _),
            &mut bmi,
            DIB_RGB_COLORS,
        );

        SelectObject(hdc, old_bmp);
        let _ = DeleteDC(hdc);
        let _ = DeleteObject(hbitmap.into());
        if co_init.is_ok() {
            CoUninitialize();
        }

        let mut rgba_pixels = Vec::with_capacity((width * height) as usize);
        for chunk in bgra_pixels.chunks_exact(4) {
            rgba_pixels.push(Rgba8Pixel {
                r: chunk[2],
                g: chunk[1],
                b: chunk[0],
                a: chunk[3],
            });
        }

        let mut buffer = SharedPixelBuffer::<Rgba8Pixel>::new(width, height);
        buffer.make_mut_slice().copy_from_slice(&rgba_pixels);

        Some(Image::from_rgba8(buffer))
    }
}

#[cfg(not(target_os = "windows"))]
pub fn extract_icon(_path: &Path) -> Option<Image> {
    None
}
