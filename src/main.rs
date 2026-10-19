#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod app;
mod branding;
mod document;
mod editing;
mod ordering;
mod pdf;
mod recovery;
mod render;
mod storage;
mod svg;

fn main() {
    std::panic::set_hook(Box::new(|info| {
        let _ = std::fs::write(
            std::env::temp_dir().join("ArtDraw-error.log"),
            format!("{info}"),
        );
        eprintln!("{info}");
    }));
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).is_some_and(|s| s == "--self-test") {
        let output = args.get(2).map(String::as_str).unwrap_or("self-test.txt");
        let result = self_test();
        let _ = std::fs::write(
            output,
            match &result {
                Ok(s) => s.clone(),
                Err(e) => format!("FAIL: {e}"),
            },
        );
        std::process::exit(if result.is_ok() { 0 } else { 1 });
    }
    if args.get(1).is_some_and(|s| s == "--pdf-proof") {
        let file = std::path::PathBuf::from(
            args.get(2)
                .map(String::as_str)
                .unwrap_or("output/pdf/ArtDraw-proof.pdf"),
        );
        let result = (|| -> Result<(), String> {
            if let Some(parent) = file.parent().filter(|p| !p.as_os_str().is_empty()) {
                std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            let doc = pdf::proof_document();
            pdf::save(&file, &doc, true)?;
            storage::save(&file.with_extension("artdraw"), &doc)?;
            Ok(())
        })();
        let _ = std::fs::write(
            file.with_extension("txt"),
            match &result {
                Ok(()) => "PASS: PDF vetorial3 páginas e fonte ArtDraw; RGB/texto em curvas".into(),
                Err(e) => format!("FAIL: {e}"),
            },
        );
        std::process::exit(if result.is_ok() { 0 } else { 1 });
    }
    let pdf_preview = args.get(1).is_some_and(|s| s == "--pdf-smoke");
    let recovery_test = args.get(1).is_some_and(|s| s == "--recovery-smoke");
    if recovery_test {
        let Some(dir) = args.get(3) else {
            eprintln!("Informe diretório isolado de recuperação");
            std::process::exit(2);
        };
        let dir = std::path::PathBuf::from(dir);
        std::fs::create_dir_all(&dir).expect("test recovery dir");
        storage::atomic_write(
            &dir.join("recovery-0-fixture.artdraw"),
            storage::encode(&document::Document::demo()).as_bytes(),
            false,
        )
        .expect("fixture");
        std::env::set_var("ARTDRAW_RECOVERY_DIR", &dir);
    }
    let interactions = args.get(1).is_some_and(|s| s == "--interaction-smoke");
    let smoke = pdf_preview
        || recovery_test
        || interactions
        || args.get(1).is_some_and(|s| s == "--gpu-smoke");
    let smoke_path = if smoke {
        Some(std::path::PathBuf::from(
            args.get(2).map(String::as_str).unwrap_or("gpu-smoke.png"),
        ))
    } else {
        None
    };
    if args.get(1).is_some_and(|s| s == "--splash-preview") {
        if let Some(splash) = branding::StartupSplash::preview() {
            splash.stage(1);
            std::thread::sleep(std::time::Duration::from_secs(45));
            splash.finish();
            std::thread::sleep(std::time::Duration::from_millis(150));
        }
        return;
    }
    let splash = (!smoke).then(branding::StartupSplash::show).flatten();
    if let Some(splash) = &splash {
        splash.stage(1);
    }
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1440., 960.])
            .with_min_inner_size([960., 640.])
            .with_maximized(!smoke)
            .with_title("ArtDraw — Editor vetorial")
            .with_icon(branding::icon()),
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };
    if let Err(error) = eframe::run_native(
        "ArtDraw",
        options,
        Box::new(move |cc| {
            Ok(Box::new(app::ArtDraw::new(
                cc,
                smoke_path,
                interactions,
                splash,
                recovery_test,
                pdf_preview,
            )?))
        }),
    ) {
        let message = format!("Não foi possível iniciar o ArtDraw: {error}\nÉ necessária uma GPU com suporte a DirectX 12 ou Vulkan e compute shaders.");
        let _ = std::fs::write(std::env::temp_dir().join("ArtDraw-error.log"), &message);
        if !smoke {
            rfd::MessageDialog::new()
                .set_title("ArtDraw")
                .set_description(&message)
                .set_level(rfd::MessageLevel::Error)
                .show();
        }
        std::process::exit(1);
    }
}
fn self_test() -> Result<String, String> {
    use vello::kurbo::{Point, Vec2};
    let doc = document::Document::demo();
    let xml = svg::export(&doc);
    let (imported, warnings) = svg::import(&xml)?;
    if imported.objects.len() != doc.objects.len() {
        return Err("SVG round-trip: contagem divergente".into());
    }
    if (imported.width - doc.width).abs() > 0.001 {
        return Err("SVG round-trip: tamanho divergente".into());
    }
    let mut object = doc.objects[1].clone();
    let before = object.bounds();
    object.translate(Vec2::new(33., 17.));
    if (object.bounds().x0 - before.x0 - 33.).abs() > 1e-6 {
        return Err("Translação inválida".into());
    }
    if doc.hit(Point::new(700., 230.), 2.) != Some(1) {
        return Err("Seleção geométrica inválida".into());
    }
    if !pdf::encode(&doc, true)?.starts_with(b"%PDF-1.4") {
        return Err("PDF vetorial inválido".into());
    }
    Ok(format!("PASS\nArtDraw 1.9.0\nObjetos SVG: {}\nTexto Unicode, curvas, seleção e translação: OK\nAvisos: {}\n", imported.objects.len(), warnings.join("; ")))
}
