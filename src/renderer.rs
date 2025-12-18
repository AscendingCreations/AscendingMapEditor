use graphics::*;
use winit::dpi::PhysicalSize;

use crate::{
    AudioCollection, ConfigData, TextureAllocation, audio::Audio, content::Content, data_types::*,
    gfx_collection::*,
};

pub struct TextCaret {
    pub visible: bool,
    pub index: Option<GfxType>,
    pub timer: f32,
}

pub struct SystemHolder {
    pub gfx: GfxCollection,
    pub renderer: GpuRenderer,
    pub size: PhysicalSize<f32>,
    pub scale: f64,
    pub resource: Box<TextureAllocation>,
    pub config: ConfigData,
    pub caret: TextCaret,
    pub audio: Audio,
}

pub struct Graphics<Controls>
where
    Controls: camera::controls::Controls,
{
    /// World Camera Controls and time. Deturmines how the world is looked at.
    pub system: System<Controls>,
    /// Atlas Groups for Textures in GPU
    pub image_atlas: AtlasSet,
    pub map_atlas: AtlasSet,
    pub text_atlas: TextAtlas,
    pub ui_atlas: AtlasSet,
    /// Rendering Buffers and other shared data.
    pub image_renderer: ImageRenderer,
    pub text_renderer: TextRenderer,
    pub mesh_renderer: Mesh2DRenderer,
    pub map_renderer: MapRenderer,
    pub light_renderer: LightRenderer,
    pub ui_renderer: RectRenderer,
}

impl<Controls> Pass for Graphics<Controls>
where
    Controls: camera::controls::Controls,
{
    fn render(&mut self, renderer: &GpuRenderer, encoder: &mut wgpu::CommandEncoder) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: renderer.frame_buffer().as_ref().expect("no frame view?"),
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.05,
                        g: 0.05,
                        b: 0.05,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: renderer.depth_buffer(),
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(0),
                    store: wgpu::StoreOp::Store,
                }),
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        // Lets set the System's Shader information here, mostly Camera, Size and Time
        pass.set_bind_group(0, self.system.bind_group(), &[]);
        // Lets set the Reusable Vertices and Indicies here.
        // This is used for each Renderer, Should be more performant since it is shared.
        pass.set_vertex_buffer(0, renderer.buffer_object.vertices());
        pass.set_index_buffer(renderer.buffer_object.indices(), wgpu::IndexFormat::Uint32);

        for layer in 0..=MAX_RENDER {
            pass.render_map(renderer, &self.map_renderer, &self.map_atlas, layer);
            pass.render_image(
                renderer,
                &self.image_renderer,
                &self.image_atlas,
                &self.system,
                layer,
            );
            pass.render_rects(
                renderer,
                &self.ui_renderer,
                &self.ui_atlas,
                &self.system,
                layer,
            );
            pass.render_text(renderer, &self.text_renderer, &self.text_atlas, layer);
            pass.render_2dmeshs(renderer, &self.mesh_renderer, &self.system, layer);
            if layer == RENDER_LIGHT {
                pass.render_lights(renderer, &self.light_renderer, RENDER_LIGHT);
            }
        }
    }
}

pub fn add_image_to_buffer<Controls>(
    content: &mut Content,
    systems: &mut SystemHolder,
    graphics: &mut Graphics<Controls>,
) where
    Controls: camera::controls::Controls,
{
    systems.gfx.image_storage.iter_mut().for_each(|(_, gfx)| {
        let visible = if let Some(visible) = gfx.data.override_visible {
            visible
        } else {
            gfx.data.visible
        };

        if visible {
            if gfx.data.debug_track {
                // Add Breakpoint to proceed with debug here
                gfx.data.debug_track = false;
            }
            graphics.image_renderer.update(
                &mut gfx.gfx,
                &mut systems.renderer,
                &mut graphics.image_atlas,
                gfx.data.layer,
            );
        }
    });
    systems.gfx.text_storage.iter_mut().for_each(|(_, gfx)| {
        let visible = if let Some(visible) = gfx.data.override_visible {
            visible
        } else {
            gfx.data.visible
        };

        if visible {
            if gfx.data.debug_track {
                // Add Breakpoint to proceed with debug here
                gfx.data.debug_track = false;
            }
            graphics
                .text_renderer
                .update(
                    &mut gfx.gfx,
                    &mut graphics.text_atlas,
                    &mut systems.renderer,
                    gfx.data.layer,
                )
                .unwrap();
        }
    });
    systems.gfx.rect_storage.iter_mut().for_each(|(_, gfx)| {
        let visible = if let Some(visible) = gfx.data.override_visible {
            visible
        } else {
            gfx.data.visible
        };

        if visible {
            if gfx.data.debug_track {
                // Add Breakpoint to proceed with debug here
                gfx.data.debug_track = false;
            }
            graphics.ui_renderer.update(
                &mut gfx.gfx,
                &mut systems.renderer,
                &mut graphics.ui_atlas,
                gfx.data.layer,
            );
        }
    });
    systems.gfx.mesh_storage.iter_mut().for_each(|(_, gfx)| {
        let visible = if let Some(visible) = gfx.data.override_visible {
            visible
        } else {
            gfx.data.visible
        };

        if visible {
            if gfx.data.debug_track {
                // Add Breakpoint to proceed with debug here
                gfx.data.debug_track = false;
            }
            graphics
                .mesh_renderer
                .update(&mut gfx.gfx, &mut systems.renderer, gfx.data.layer);
        }
    });
    systems.gfx.light_storage.iter_mut().for_each(|(_, gfx)| {
        let visible = if let Some(visible) = gfx.data.override_visible {
            visible
        } else {
            gfx.data.visible
        };

        if visible {
            if gfx.data.debug_track {
                // Add Breakpoint to proceed with debug here
                gfx.data.debug_track = false;
            }
            graphics
                .light_renderer
                .update(&mut gfx.gfx, &mut systems.renderer, gfx.data.layer);
        }
    });

    graphics.map_renderer.update(
        &mut content.map_view.map,
        &mut systems.renderer,
        &mut graphics.map_atlas,
        [0, 1],
    );
    for map in content.map_view.linked_map.iter_mut() {
        graphics.map_renderer.update(
            &mut map.map,
            &mut systems.renderer,
            &mut graphics.map_atlas,
            [0, 1],
        );
    }
}
