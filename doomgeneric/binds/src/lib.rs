use std::{
    ffi::CStr,
    fs::create_dir,
    slice,
    sync::OnceLock,
    thread,
    time::{Duration, Instant},
};

use image::RgbImage;

pub type Pixel = u32;

unsafe extern "C" {
    static DG_ScreenBuffer: *const Pixel;
}

static START: OnceLock<Instant> = OnceLock::new();

#[unsafe(no_mangle)]
pub extern "C" fn DG_Init() {
    START.set(Instant::now()).ok();
    create_dir("frames/").ok();
    println!("DG_Init: called");
}

#[unsafe(no_mangle)]
pub extern "C" fn DG_DrawFrame() {
    if unsafe { DG_ScreenBuffer }.is_null() {
        println!("DG_DrawFrame: DG_ScreenBuffer is null");
        return;
    }

    let buffer = unsafe { slice::from_raw_parts(DG_ScreenBuffer, 640 * 400) };

    let img: Vec<_> = buffer
        .iter()
        .flat_map(|p| {
            let r = (p >> 16) & 255;
            let g = (p >> 8) & 255;
            let b = p & 255;

            [r as u8, g as u8, b as u8]
        })
        .collect();

    let img = RgbImage::from_vec(640, 400, img).unwrap();

    let file = format!("frames/{}.png", get_elapsed_ms());

    img.save(file).unwrap();
}

#[unsafe(no_mangle)]
pub extern "C" fn DG_SleepMs(ms: u32) {
    println!("DG_SleepMs: called for {ms} ms");
    thread::sleep(Duration::from_millis(ms as u64));
}

#[unsafe(no_mangle)]
pub extern "C" fn DG_GetTicksMs() -> u32 {
    let passed = Instant::now()
        .duration_since(*START.get().unwrap())
        .as_millis() as u32;

    println!("DG_GetTicksMs: passed {passed} ms");

    passed
}

/// # Safety
/// confia papai
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DG_GetKey(pressed: *mut i32, doom_key: *const u8) -> i32 {
    let key = unsafe { *doom_key } as char;
    println!("DG_GetKey: Checking {key}");
    unsafe { *pressed = 0 }
    0
}

/// # Safety
/// confia papai
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DG_SetWindowTitle(title: *const i8) {
    let title = unsafe { CStr::from_ptr(title) }.to_str().unwrap();

    println!("DG_SetWindowTitle: Changing title to {title}");
}

fn get_elapsed_ms() -> u128 {
    START.get().unwrap().elapsed().as_millis()
}
