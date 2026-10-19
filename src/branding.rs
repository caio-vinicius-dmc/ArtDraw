//! Lightweight Windows splash shown before eframe or the GPU starts.
//! Bitmap is embedded: startup does not depend on files beside the executable.
use std::ffi::c_void;
use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
type Handle = isize;
#[repr(C)]
#[derive(Clone, Copy, Default)]
struct Rect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}
#[repr(C)]
struct WindowClass {
    style: u32,
    proc: Option<unsafe extern "system" fn(Handle, u32, usize, isize) -> isize>,
    class_extra: i32,
    window_extra: i32,
    instance: Handle,
    icon: Handle,
    cursor: Handle,
    background: Handle,
    menu: *const u16,
    name: *const u16,
}
#[repr(C)]
#[derive(Default)]
struct Message {
    window: Handle,
    message: u32,
    wparam: usize,
    lparam: isize,
    time: u32,
    point: [i32; 2],
    private: u32,
}
#[repr(C)]
struct Paint {
    dc: Handle,
    erase: i32,
    rect: Rect,
    restore: i32,
    update: i32,
    reserved: [u8; 32],
}
#[link(name = "user32")]
extern "system" {
    fn RegisterClassW(class: *const WindowClass) -> u16;
    fn CreateWindowExW(
        ex: u32,
        class: *const u16,
        title: *const u16,
        style: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        parent: Handle,
        menu: Handle,
        instance: Handle,
        param: *mut c_void,
    ) -> Handle;
    fn DefWindowProcW(window: Handle, message: u32, wparam: usize, lparam: isize) -> isize;
    fn SetWindowLongPtrW(window: Handle, index: i32, value: isize) -> isize;
    fn GetWindowLongPtrW(window: Handle, index: i32) -> isize;
    fn GetMessageW(message: *mut Message, window: Handle, min: u32, max: u32) -> i32;
    fn TranslateMessage(message: *const Message) -> i32;
    fn DispatchMessageW(message: *const Message) -> isize;
    fn ShowWindow(window: Handle, command: i32) -> i32;
    fn UpdateWindow(window: Handle) -> i32;
    fn PostMessageW(window: Handle, message: u32, wparam: usize, lparam: isize) -> i32;
    fn PostQuitMessage(code: i32);
    fn DestroyWindow(window: Handle) -> i32;
    fn GetSystemMetrics(index: i32) -> i32;
    fn GetDpiForSystem() -> u32;
    fn SetTimer(window: Handle, id: usize, interval: u32, callback: *const c_void) -> usize;
    fn KillTimer(window: Handle, id: usize) -> i32;
    fn InvalidateRect(window: Handle, rect: *const Rect, erase: i32) -> i32;
    fn BeginPaint(window: Handle, paint: *mut Paint) -> Handle;
    fn EndPaint(window: Handle, paint: *const Paint) -> i32;
    fn FillRect(dc: Handle, rect: *const Rect, brush: Handle) -> i32;
    fn DrawTextW(dc: Handle, text: *const u16, count: i32, rect: *mut Rect, format: u32) -> i32;
    fn LoadCursorW(instance: Handle, name: *const u16) -> Handle;
}
#[link(name = "gdi32")]
extern "system" {
    fn CreateSolidBrush(color: u32) -> Handle;
    fn DeleteObject(object: Handle) -> i32;
    fn SetBkMode(dc: Handle, mode: i32) -> i32;
    fn SetTextColor(dc: Handle, color: u32) -> u32;
    fn SelectObject(dc: Handle, object: Handle) -> Handle;
    fn CreateFontW(
        height: i32,
        width: i32,
        escapement: i32,
        orientation: i32,
        weight: i32,
        italic: u32,
        underline: u32,
        strike: u32,
        charset: u32,
        out_precision: u32,
        clip_precision: u32,
        quality: u32,
        pitch: u32,
        face: *const u16,
    ) -> Handle;
    fn StretchDIBits(
        dc: Handle,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        sx: i32,
        sy: i32,
        sw: i32,
        sh: i32,
        bits: *const c_void,
        info: *const c_void,
        usage: u32,
        rop: u32,
    ) -> i32;
    fn SetStretchBltMode(dc: Handle, mode: i32) -> i32;
}
#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> Handle;
}
const BITMAP: &[u8] = include_bytes!("../assets/branding/splash.bmp");
const READY: u32 = 0x8001;
const STAGE: u32 = 0x8002;
struct State {
    began: Instant,
    ready: bool,
    stage: usize,
    scale: f64,
}
/// Owns the window until finish is called; Drop also closes it on startup error.
pub struct StartupSplash {
    window: Handle,
    handed_off: bool,
}
impl StartupSplash {
    pub fn show() -> Option<Self> {
        Self::start(false)
    }
    pub fn preview() -> Option<Self> {
        Self::start(true)
    }
    fn start(preview: bool) -> Option<Self> {
        let (tx, rx) = mpsc::sync_channel(1);
        thread::Builder::new()
            .name("ArtDraw splash".into())
            .spawn(move || unsafe {
                let instance = GetModuleHandleW(std::ptr::null());
                let name = wide("ArtDrawStartupSplash");
                let class = WindowClass {
                    style: 0,
                    proc: Some(window_proc),
                    class_extra: 0,
                    window_extra: 0,
                    instance,
                    icon: 0,
                    cursor: LoadCursorW(0, 32512usize as *const u16),
                    background: 0,
                    menu: std::ptr::null(),
                    name: name.as_ptr(),
                };
                RegisterClassW(&class);
                let scale = GetDpiForSystem().max(96) as f64 / 96.;
                let width = (680. * scale) as i32;
                let height = (470. * scale) as i32;
                let mut state = Box::new(State {
                    began: Instant::now(),
                    ready: false,
                    stage: 0,
                    scale,
                });
                let window = CreateWindowExW(
                    (if preview { 0x00040000 } else { 0x00000080 }) | 0x00000008,
                    name.as_ptr(),
                    wide("ArtDraw — Carregando").as_ptr(),
                    0x80000000 | 0x00800000,
                    (GetSystemMetrics(0) - width) / 2,
                    (GetSystemMetrics(1) - height) / 2,
                    width,
                    height,
                    0,
                    0,
                    instance,
                    std::ptr::null_mut(),
                );
                if window == 0 {
                    let _ = tx.send(0);
                    return;
                }
                SetWindowLongPtrW(window, -21, (&mut *state as *mut State) as isize);
                SetTimer(window, 1, 40, std::ptr::null());
                ShowWindow(window, 4);
                UpdateWindow(window);
                if tx.send(window).is_err() {
                    DestroyWindow(window);
                    return;
                }
                let mut message = Message::default();
                while GetMessageW(&mut message, 0, 0, 0) > 0 {
                    TranslateMessage(&message);
                    DispatchMessageW(&message);
                }
            })
            .ok()?;
        let window = rx.recv_timeout(Duration::from_secs(3)).ok()?;
        (window != 0).then_some(Self {
            window,
            handed_off: false,
        })
    }
    pub fn stage(&self, stage: usize) {
        unsafe {
            PostMessageW(self.window, STAGE, stage, 0);
        }
    }
    pub fn finish(mut self) {
        unsafe {
            PostMessageW(self.window, READY, 0, 0);
        }
        self.handed_off = true;
    }
}
impl Drop for StartupSplash {
    fn drop(&mut self) {
        if !self.handed_off {
            unsafe {
                PostMessageW(self.window, 0x0010, 0, 0);
            }
        }
    }
}
fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}
fn rgb(r: u32, g: u32, b: u32) -> u32 {
    r | (g << 8) | (b << 16)
}
unsafe fn fill(dc: Handle, rect: Rect, color: u32) {
    let brush = CreateSolidBrush(color);
    FillRect(dc, &rect, brush);
    DeleteObject(brush);
}
unsafe fn text(dc: Handle, s: &str, mut rect: Rect, size: i32, bold: bool, color: u32) {
    let font = CreateFontW(
        -size,
        0,
        0,
        0,
        if bold { 600 } else { 400 },
        0,
        0,
        0,
        1,
        0,
        0,
        5,
        0,
        wide("Segoe UI").as_ptr(),
    );
    let previous = SelectObject(dc, font);
    SetBkMode(dc, 1);
    SetTextColor(dc, color);
    let value = wide(s);
    DrawTextW(dc, value.as_ptr(), -1, &mut rect, 0x0001 | 0x0020 | 0x0004);
    SelectObject(dc, previous);
    DeleteObject(font);
}
unsafe extern "system" fn window_proc(
    window: Handle,
    message: u32,
    wparam: usize,
    lparam: isize,
) -> isize {
    let state = GetWindowLongPtrW(window, -21) as *mut State;
    match message {
        0x0010 => {
            DestroyWindow(window);
            return 0;
        }
        0x0002 => {
            KillTimer(window, 1);
            PostQuitMessage(0);
            return 0;
        }
        0x0014 => return 1,
        _ => {}
    }
    if state.is_null() {
        return DefWindowProcW(window, message, wparam, lparam);
    }
    let state = &mut *state;
    match message {
        READY => {
            state.ready = true;
            state.stage = 2;
            0
        }
        STAGE => {
            state.stage = wparam.min(2);
            0
        }
        0x0113 => {
            if state.ready && state.began.elapsed() >= Duration::from_millis(1100) {
                DestroyWindow(window);
            } else {
                InvalidateRect(window, std::ptr::null(), 0);
            }
            0
        }
        0x000F => {
            let mut paint: Paint = std::mem::zeroed();
            let dc = BeginPaint(window, &mut paint);
            let s = |v: f64| (v * state.scale).round() as i32;
            fill(
                dc,
                Rect {
                    left: 0,
                    top: 0,
                    right: s(680.),
                    bottom: s(470.),
                },
                rgb(255, 255, 255),
            );
            let offset = u32::from_le_bytes(BITMAP[10..14].try_into().unwrap()) as usize;
            SetStretchBltMode(dc, 4);
            StretchDIBits(
                dc,
                s(20.),
                s(0.),
                s(640.),
                s(358.),
                0,
                0,
                640,
                358,
                BITMAP[offset..].as_ptr().cast(),
                BITMAP[14..].as_ptr().cast(),
                0,
                0x00CC0020,
            );
            text(
                dc,
                "ArtDraw",
                Rect {
                    left: s(25.),
                    top: s(329.),
                    right: s(655.),
                    bottom: s(376.),
                },
                s(34.),
                true,
                rgb(36, 51, 33),
            );
            let status = match state.stage {
                0 => "Iniciando o editor vetorial…",
                1 => "Preparando a interface e a aceleração gráfica…",
                _ => "Preparando a área de trabalho…",
            };
            text(
                dc,
                status,
                Rect {
                    left: s(25.),
                    top: s(379.),
                    right: s(655.),
                    bottom: s(407.),
                },
                s(14.),
                false,
                rgb(82, 97, 77),
            );
            let track = Rect {
                left: s(64.),
                top: s(422.),
                right: s(616.),
                bottom: s(426.),
            };
            fill(dc, track, rgb(231, 239, 226));
            let distance = state.began.elapsed().as_millis() as f64 / 1000. * 0.7;
            let phase = distance.fract();
            let x = 64. + phase * 432.;
            fill(
                dc,
                Rect {
                    left: s(x),
                    top: s(422.),
                    right: s(x + 120.),
                    bottom: s(426.),
                },
                rgb(67, 165, 27),
            );
            text(
                dc,
                "VETORES  ·  CRIATIVIDADE  ·  OFFLINE",
                Rect {
                    left: s(25.),
                    top: s(439.),
                    right: s(655.),
                    bottom: s(459.),
                },
                s(10.),
                false,
                rgb(110, 121, 106),
            );
            EndPaint(window, &paint);
            0
        }
        _ => DefWindowProcW(window, message, wparam, lparam),
    }
}

pub fn icon() -> eframe::egui::IconData {
    let image = image::load_from_memory(include_bytes!("../assets/branding/app.png"))
        .expect("embedded PNG icon")
        .into_rgba8();
    eframe::egui::IconData {
        width: image.width(),
        height: image.height(),
        rgba: image.into_raw(),
    }
}
