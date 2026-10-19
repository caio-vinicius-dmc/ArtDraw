use crate::document::{Color, Document, Object, Page};
use eframe::{egui, egui_wgpu};
use vello::wgpu;
use vello::{
    kurbo::{Affine, Rect, Stroke},
    peniko::{Color as VColor, Fill},
    AaConfig, AaSupport, RenderParams, Renderer, RendererOptions, Scene,
};

pub struct CanvasRenderer {
    state: egui_wgpu::RenderState,
    renderer: Renderer,
    texture: Option<(wgpu::Texture, wgpu::TextureView, egui::TextureId, u32, u32)>,
    last: Option<(Page, Option<Object>, [f64; 3], Color)>,
    pub adapter: String,
    pub frames: u64,
}
fn color(c: Color) -> VColor {
    VColor::from_rgba8(c[0], c[1], c[2], c[3])
}
impl CanvasRenderer {
    pub fn new(state: egui_wgpu::RenderState) -> Result<Self, String> {
        let info = state.adapter.get_info();
        let renderer = Renderer::new(
            &state.device,
            RendererOptions {
                antialiasing_support: AaSupport::area_only(),
                ..Default::default()
            },
        )
        .map_err(|e| format!("Vello: {e}"))?;
        Ok(Self {
            state,
            renderer,
            texture: None,
            last: None,
            adapter: format!("{} · {:?}", info.name, info.backend),
            frames: 0,
        })
    }
    pub fn paint(
        &mut self,
        ui: &egui::Ui,
        rect: egui::Rect,
        doc: &Document,
        draft: Option<&Object>,
        origin: egui::Pos2,
        zoom: f64,
        background: Color,
    ) -> Result<(), String> {
        let ppp = f64::from(ui.ctx().pixels_per_point());
        let max = self.state.device.limits().max_texture_dimension_2d;
        let width = ((f64::from(rect.width()) * ppp).round() as u32).clamp(1, max);
        let height = ((f64::from(rect.height()) * ppp).round() as u32).clamp(1, max);
        let resized = self
            .texture
            .as_ref()
            .is_none_or(|t| t.3 != width || t.4 != height);
        if resized {
            if let Some((_, _, id, _, _)) = self.texture.take() {
                self.state.renderer.write().free_texture(&id);
            }
            let texture = self.state.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("ArtDraw Vello canvas"),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::STORAGE_BINDING
                    | wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::COPY_SRC,
                view_formats: &[wgpu::TextureFormat::Rgba8UnormSrgb],
            });
            let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
            let display_view = texture.create_view(&wgpu::TextureViewDescriptor {
                format: Some(wgpu::TextureFormat::Rgba8UnormSrgb),
                usage: Some(wgpu::TextureUsages::TEXTURE_BINDING),
                ..Default::default()
            });
            let id = self.state.renderer.write().register_native_texture(
                &self.state.device,
                &display_view,
                wgpu::FilterMode::Linear,
            );
            self.texture = Some((texture, view, id, width, height));
        }
        let ox = f64::from(origin.x - rect.min.x) * ppp;
        let oy = f64::from(origin.y - rect.min.y) * ppp;
        let view = [ox, oy, zoom * ppp];
        let changed = resized
            || self.last.as_ref().is_none_or(|(d, p, v, bg)| {
                d != &**doc || p.as_ref() != draft || *v != view || *bg != background
            });
        if changed {
            let mut scene = Scene::new();
            let transform = Affine::translate((ox, oy)) * Affine::scale(zoom * ppp);
            scene.fill(
                Fill::NonZero,
                transform * Affine::translate((5., 7.)),
                color([37, 40, 47, 255]),
                None,
                &Rect::new(0., 0., doc.width, doc.height),
            );
            scene.fill(
                Fill::NonZero,
                transform,
                color([255, 255, 255, 255]),
                None,
                &Rect::new(0., 0., doc.width, doc.height),
            );
            for object in doc
                .paint_order()
                .into_iter()
                .filter(|&i| doc.visible(i))
                .map(|i| &doc.objects[i])
                .chain(draft)
            {
                let path = object.local_path();
                let t = transform * object.transform;
                if let Some(c) = object.fill {
                    scene.fill(
                        if object.evenodd {
                            Fill::EvenOdd
                        } else {
                            Fill::NonZero
                        },
                        t,
                        color(c),
                        None,
                        &path,
                    );
                }
                if let Some(c) = object.stroke {
                    if object.stroke_width > 0. {
                        scene.stroke(
                            &Stroke::new(object.stroke_width)
                                .with_join(vello::kurbo::Join::Round)
                                .with_caps(vello::kurbo::Cap::Round),
                            t,
                            color(c),
                            None,
                            &path,
                        );
                    }
                }
            }
            let target = &self.texture.as_ref().unwrap().1;
            self.renderer
                .render_to_texture(
                    &self.state.device,
                    &self.state.queue,
                    &scene,
                    target,
                    &RenderParams {
                        base_color: color(background),
                        width,
                        height,
                        antialiasing_method: AaConfig::Area,
                    },
                )
                .map_err(|e| format!("Renderização Vello: {e}"))?;
            self.frames += 1;
            self.last = Some(((**doc).clone(), draft.cloned(), view, background));
        }
        let id = self.texture.as_ref().unwrap().2;
        ui.painter().image(
            id,
            rect,
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1., 1.)),
            egui::Color32::WHITE,
        );
        Ok(())
    }
}
