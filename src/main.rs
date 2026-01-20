#![allow(dead_code, clippy::collapsible_match, unused_imports)]
use backtrace::Backtrace;
use camera::{
    Projection,
    controls::{Controls, FlatControls, FlatSettings},
};
use cosmic_text::{Attrs, Metrics};
use graphics::{
    wgpu::{
        BackendOptions, Dx12BackendOptions, ExperimentalFeatures, MemoryBudgetThresholds,
        NoopBackendOptions, wgt::Dx12SwapchainKind,
    },
    *,
};
use input::{Bindings, InputHandler, Key, MouseAxis};
use log::{Level, LevelFilter, Metadata, Record, error, info, warn};
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{self, File},
    io::{Read, Write, prelude::*},
    iter, panic,
    sync::Arc,
    time::{Duration, Instant},
};
use time::FrameTime;
use wgpu::{Backends, Dx12Compiler, InstanceDescriptor, InstanceFlags};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::*,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    platform::windows::WindowAttributesExtWindows,
    window::{WindowAttributes, WindowButtons},
};

const WAIT_TIME: std::time::Duration = std::time::Duration::from_millis(20);

mod audio;
mod config;
mod content;
mod data_types;
mod database;
mod gfx_collection;
mod renderer;
mod resource;

use audio::*;
use config::*;
use content::*;
use data_types::*;
use database::*;
use gfx_collection::*;
use renderer::*;
use resource::*;

use crate::content::widget::{Alert, AlertBuilder, AlertIndex, Tooltip};

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
enum Axis {
    Forward,
    Sideward,
    Yaw,
    Pitch,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
enum Action {
    None,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MousePress {
    None,
    LeftClick,
    RightClick,
    MiddleClick,
}

// creates a static global logger type for setting the logger
static MY_LOGGER: MyLogger = MyLogger(Level::Debug);

struct MyLogger(pub Level);

impl log::Log for MyLogger {
    // checks if it can log these types of events.
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.0
    }

    // This logs to a panic file. This is so we can see
    // Errors and such if a program crashes in full render mode.
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let msg = format!("{} - {}\n", record.level(), record.args());
            println!("{}", &msg);

            let mut file = match File::options()
                .append(true)
                .create(true)
                .open("map_editor_log.txt")
            {
                Ok(v) => v,
                Err(_) => return,
            };

            let _ = file.write(msg.as_bytes());
        }
    }
    fn flush(&self) {}
}

#[allow(clippy::large_enum_variant)]
enum Runner {
    Loading,
    Ready {
        systems: SystemHolder,
        graphics: Graphics<FlatControls>,
        content: Content,
        input_handler: InputHandler<Action, Axis>,
        frame_time: FrameTime,
        time: f32,
        fps: u32,
        mouse_pos: PhysicalPosition<f64>,
        mouse_press: MousePress,
        loop_timer: LoopTimer,
        tooltip: Tooltip,
        alert: Alert,
    },
}

impl winit::application::ApplicationHandler for Runner {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Self::Loading = self {
            info!("loading initiation");
            let win_attrs = WindowAttributes::default()
                .with_active(false)
                .with_visible(false)
                .with_inner_size(PhysicalSize::new(1024.0, 768.0))
                .with_min_inner_size(PhysicalSize::new(800.0, 400.0))
                .with_title("Map Editor")
                .with_enabled_buttons(WindowButtons::all());

            // Builds the Windows that will be rendered too.
            let window = Arc::new(event_loop.create_window(win_attrs).expect("Create window"));

            info!("after window initiation");

            // Generates an Instance for WGPU. Sets WGPU to be allowed on all possible supported backends
            // These are DX12, DX11, Vulkan, Metal and Gles. if none of these work on a system they cant
            // play the game basically.
            let instance = wgpu::Instance::new(&InstanceDescriptor {
                backends: Backends::all(),
                flags: InstanceFlags::empty(),
                backend_options: BackendOptions {
                    gl: wgpu::GlBackendOptions {
                        gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
                        fence_behavior: wgpu::GlFenceBehavior::Normal,
                    },
                    dx12: wgpu::Dx12BackendOptions {
                        shader_compiler: Dx12Compiler::StaticDxc,
                        latency_waitable_object:
                            wgpu::wgt::Dx12UseFrameLatencyWaitableObject::DontWait,
                        presentation_system: Dx12SwapchainKind::default(),
                    },
                    noop: NoopBackendOptions::default(),
                },
                memory_budget_thresholds: MemoryBudgetThresholds::default(),
            });

            info!("after wgpu instance initiation");

            // This is used to ensure the GPU can load the correct.
            let compatible_surface = instance.create_surface(window.clone()).unwrap();

            info!("after compatible initiation");
            print!("{:?}", &compatible_surface);
            // This creates the Window Struct and Device struct that holds all the rendering information
            // we need to render to the screen. Window holds most of the window information including
            // the surface type. device includes the queue and GPU device for rendering.
            // This then adds gpu_window and gpu_device and creates our renderer type. for easy passing of window, device and font system.
            let mut renderer = futures::executor::block_on(instance.create_device(
                window,
                //used to find adapters
                AdapterOptions {
                    allowed_backends: Backends::all(),
                    power: AdapterPowerSettings::HighPower,
                    compatible_surface: Some(compatible_surface),
                },
                // used to deturmine which adapters support our special limits or features for our backends.
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::default(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                    memory_hints: wgpu::MemoryHints::Performance,
                    trace: wgpu::Trace::Off,
                    experimental_features: ExperimentalFeatures::disabled(),
                },
                // How we are presenting the screen which causes it to either clip to a FPS limit or be unlimited.
                wgpu::PresentMode::AutoVsync,
                EnabledPipelines::all(),
            ))
            .unwrap();

            info!("after renderer initiation");
            // we print the GPU it decided to use here for testing purposes.
            println!("{:?}", renderer.adapter().get_info());

            // We generate Texture atlases to use with out types.
            let mut image_atlas = AtlasSet::new(
                &mut renderer,
                wgpu::TextureFormat::Rgba8UnormSrgb,
                true,
                8192,
            );
            let mut map_atlas = AtlasSet::new(
                &mut renderer,
                wgpu::TextureFormat::Rgba8UnormSrgb,
                true,
                2048,
            );
            let ui_atlas = AtlasSet::new(
                &mut renderer,
                wgpu::TextureFormat::Rgba8UnormSrgb,
                true,
                256,
            );
            let text_atlas = TextAtlas::new(&mut renderer, 1024).unwrap();

            // get the screen size.
            let size = renderer.size();
            let mat = Mat4::from_translation(Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            });

            // get the Scale factor the pc currently is using for upscaling or downscaling the rendering.
            let scale = renderer
                .window()
                .current_monitor()
                .unwrap()
                .scale_factor()
                .clamp(1.0, 1.5);

            // Load textures image
            let resource = Box::new(
                TextureAllocation::new(&mut image_atlas, &mut map_atlas, &renderer).unwrap(),
            );

            let mut audio = Audio::new(0.15).unwrap();
            audio.set_effect_volume(1.0);
            audio.set_music_volume(1.0);

            // Compile all rendering data in one type for quick access and passing
            let mut systems = SystemHolder {
                gfx: GfxCollection::new(),
                renderer,
                size,
                scale,
                resource,
                config: load_config(),
                caret: TextCaret {
                    visible: false,
                    index: None,
                    timer: 0.0,
                },
                audio,
            };

            // We establish the different renderers here to load their data up to use them.
            let text_renderer = TextRenderer::new(&systems.renderer).unwrap();
            let mesh_renderer = Mesh2DRenderer::new(&systems.renderer).unwrap();
            let image_renderer = ImageRenderer::new(&systems.renderer).unwrap();
            let mut map_renderer = MapRenderer::new(&mut systems.renderer, 81).unwrap();
            let light_renderer = LightRenderer::new(&mut systems.renderer).unwrap();
            let ui_renderer = RectRenderer::new(&systems.renderer).unwrap();

            // Initiate map editor data
            let content = Content::new(&mut systems, &mut map_renderer).unwrap();
            let tooltip = Tooltip::new(&mut systems);
            let mut alert = Alert::new();

            if is_recovery_map_file_exist() {
                alert.show_alert(
                    &mut systems,
                    AlertBuilder::new_confirm(
                        "Recovery File",
                        "Previous map file has been recovered, would you like to load this map?",
                    )
                    .with_index(AlertIndex::LoadRecoveryFile),
                );
            }

            // setup our system which includes Camera and projection as well as our controls.
            // for the camera.
            let mut system = System::new(
                &mut systems.renderer,
                Projection::Orthographic {
                    left: 0.0,
                    right: size.width,
                    bottom: 0.0,
                    top: size.height,
                    near: 1.0,
                    far: -100.0,
                },
                FlatControls::new(FlatSettings {
                    zoom: systems.config.zoom,
                }),
                [size.width, size.height],
            );
            system.set_view(CameraView::SubView1, mat, 1.0);

            // Allow the window to be seen. hiding it then making visible speeds up
            // load times.
            systems.renderer.window().set_visible(true);

            // add everything into our convience type for quicker access and passing.
            let graphics = Graphics {
                system,
                image_atlas,
                text_atlas,
                map_atlas,
                ui_atlas,
                image_renderer,
                text_renderer,
                map_renderer,
                light_renderer,
                ui_renderer,
                mesh_renderer,
            };

            systems.renderer.window().set_visible(true);

            *self = Self::Ready {
                systems,
                graphics,
                content,
                input_handler: InputHandler::new(
                    Bindings::<Action, Axis>::new(),
                    Duration::from_millis(250),
                ),
                frame_time: FrameTime::new(),
                time: 0.0f32,
                fps: 0u32,
                mouse_pos: PhysicalPosition::new(0.0, 0.0),
                mouse_press: MousePress::None,
                loop_timer: LoopTimer::default(),
                tooltip,
                alert,
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if let Self::Ready {
            systems,
            graphics,
            content,
            input_handler,
            frame_time,
            time,
            fps,
            mouse_pos,
            mouse_press,
            loop_timer,
            tooltip,
            alert,
        } = self
        {
            frame_time.update();
            let seconds = frame_time.seconds();

            if window_id == systems.renderer.window().id()
                && event == WindowEvent::CloseRequested
                && !content.data.exiting_save
            {
                alert.show_alert(
                    systems,
                    AlertBuilder::new_confirm(
                        "Exit",
                        "Are you sure that you want to exit the editor?",
                    )
                    .with_width(300)
                    .with_index(AlertIndex::ExitEditor),
                );
                return;
            }

            // update our inputs.
            input_handler.window_updates(&event);

            for input in input_handler.events() {
                match input {
                    input::InputEvent::KeyInput { key, pressed, .. } => {
                        handle_key_input(
                            &key, pressed, content, systems, alert, event_loop, seconds,
                        )
                        .unwrap();
                    }
                    input::InputEvent::MouseWheel { amount, axis } => {
                        if axis == MouseAxis::Vertical {
                            handle_mouse_wheel(
                                systems,
                                graphics,
                                amount,
                                Vec2::new(mouse_pos.x as f32, mouse_pos.y as f32),
                                content,
                            );
                        }
                    }
                    input::InputEvent::MouseButton { button, pressed } => {
                        if let Some(mouseinput) = if pressed {
                            if button == MouseButton::Left {
                                *mouse_press = MousePress::LeftClick;
                                Some(MouseInputType::LeftDown)
                            } else if button == MouseButton::Right {
                                *mouse_press = MousePress::RightClick;
                                Some(MouseInputType::RightDown)
                            } else if button == MouseButton::Middle {
                                *mouse_press = MousePress::MiddleClick;
                                Some(MouseInputType::MiddleDown)
                            } else {
                                None
                            }
                        } else if *mouse_press != MousePress::None {
                            *mouse_press = MousePress::None;
                            Some(MouseInputType::Release)
                        } else {
                            None
                        } {
                            handle_input(
                                systems,
                                graphics,
                                mouseinput,
                                Vec2::new(mouse_pos.x as f32, mouse_pos.y as f32),
                                content,
                                tooltip,
                                alert,
                                event_loop,
                                seconds,
                            )
                            .unwrap();
                        }
                    }
                    input::InputEvent::MousePosition { x, y } => {
                        *mouse_pos = PhysicalPosition { x, y };

                        let mouse_input = if *mouse_press != MousePress::None {
                            if *mouse_press == MousePress::LeftClick {
                                MouseInputType::LeftDownMove
                            } else if *mouse_press == MousePress::MiddleClick {
                                MouseInputType::MiddleDownMove
                            } else {
                                MouseInputType::RightDownMove
                            }
                        } else {
                            MouseInputType::Move
                        };

                        handle_input(
                            systems,
                            graphics,
                            mouse_input,
                            Vec2::new(mouse_pos.x as f32, mouse_pos.y as f32),
                            content,
                            tooltip,
                            alert,
                            event_loop,
                            seconds,
                        )
                        .unwrap();
                    }
                    input::InputEvent::MouseButtonAction(action) => {
                        if let input::MouseButtonAction::Double(mousebutton) = action {
                            handle_input(
                                systems,
                                graphics,
                                if mousebutton == MouseButton::Left {
                                    MouseInputType::DoubleLeftDown
                                } else {
                                    MouseInputType::DoubleRightDown
                                },
                                Vec2::new(mouse_pos.x as f32, mouse_pos.y as f32),
                                content,
                                tooltip,
                                alert,
                                event_loop,
                                seconds,
                            )
                            .unwrap();
                        }
                    }
                    _ => {}
                }
            }

            // update our renderer based on events here
            if !systems.renderer.update(&event).unwrap() {
                return;
            }

            // get the current window size so we can see if we need to resize the renderer.
            let new_size = systems.renderer.size();

            if systems.size != new_size {
                systems.size = new_size;

                // Reset screen size for the Surface here.
                graphics.system.set_projection(Projection::Orthographic {
                    left: 0.0,
                    right: new_size.width,
                    bottom: 0.0,
                    top: new_size.height,
                    near: 1.0,
                    far: -100.0,
                });

                systems.renderer.update_depth_texture();

                content.screen_resize(systems);
            }

            tooltip.handle_tooltip_logic(systems, seconds);
            editor_loop(
                systems,
                &mut graphics.map_renderer,
                content,
                seconds,
                loop_timer,
            )
            .unwrap();

            if let Some(gfx_index) = systems.caret.index
                && systems.caret.timer <= seconds
            {
                systems.caret.visible = !systems.caret.visible;
                systems.caret.timer = seconds + 0.35;
                systems.gfx.set_visible(&gfx_index, systems.caret.visible);
            }

            // update our systems data to the gpu. this is the Camera in the shaders.
            graphics.system.update(&systems.renderer, frame_time);

            // update our systems data to the gpu. this is the Screen in the shaders.
            graphics
                .system
                .update_screen(&systems.renderer, [new_size.width, new_size.height]);

            // This adds the Image data to the Buffer for rendering.
            add_image_to_buffer(content, systems, graphics);

            // this cycles all the Image's in the Image buffer by first putting them in rendering order
            // and then uploading them to the GPU if they have moved or changed in any way. clears the
            // Image buffer for the next render pass. Image buffer only holds the ID's and Sortign info
            // of the finalized Indicies of each Image.
            graphics.image_renderer.finalize(&mut systems.renderer);
            graphics.text_renderer.finalize(&mut systems.renderer);
            graphics.map_renderer.finalize(&mut systems.renderer);
            graphics.light_renderer.finalize(&mut systems.renderer);
            graphics.ui_renderer.finalize(&mut systems.renderer);
            graphics.mesh_renderer.finalize(&mut systems.renderer);

            // Start encoding commands. this stores all the rendering calls for execution when
            // finish is called.
            let mut encoder =
                systems
                    .renderer
                    .device()
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("command encoder"),
                    });

            // Run the render pass. for the games renderer
            graphics.render(&systems.renderer, &mut encoder);

            // Submit our command queue. for it to upload all the changes that were made.
            // Also tells the system to begin running the commands on the GPU.
            systems
                .renderer
                .queue()
                .submit(std::iter::once(encoder.finish()));

            if *time < seconds {
                *fps = 0u32;
                *time = seconds + 1.0;
            }
            *fps += 1;

            systems.renderer.window().pre_present_notify();
            systems.renderer.present().unwrap();

            // These clear the Last used image tags.
            //Can be used later to auto unload things not used anymore if ram/gpu ram becomes a issue.
            match *fps {
                1 => graphics.image_atlas.trim(),
                2 => graphics.map_atlas.trim(),
                3 => graphics.text_atlas.trim(),
                4 => graphics.ui_atlas.trim(),
                5 => systems.renderer.font_sys.shape_run_cache.trim(1024),
                _ => {}
            }
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: DeviceEvent,
    ) {
        if let Self::Ready {
            systems: _,
            graphics: _,
            content: _,
            input_handler,
            frame_time: _,
            time: _,
            fps: _,
            mouse_pos: _,
            mouse_press: _,
            loop_timer: _,
            tooltip: _,
            alert: _,
        } = self
        {
            input_handler.device_updates(&event);
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Self::Ready {
            systems,
            graphics: _,
            content: _,
            input_handler: _,
            frame_time: _,
            time: _,
            fps: _,
            mouse_pos: _,
            mouse_press: _,
            loop_timer: _,
            tooltip: _,
            alert: _,
        } = self
        {
            systems.renderer.window().request_redraw();
        }

        event_loop.set_control_flow(ControlFlow::WaitUntil(
            std::time::Instant::now() + WAIT_TIME,
        ));
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Create logger to output to a File
    log::set_logger(&MY_LOGGER).unwrap();
    // Set the Max level we accept logging to the file for.
    log::set_max_level(LevelFilter::Info);

    info!("starting up");

    // Create the directory for our map data
    fs::create_dir_all("./data/maps/")?;
    fs::create_dir_all("./temp/")?;
    fs::create_dir_all("./mapeditor/images/")?;
    fs::create_dir_all("./mapeditor/data/presets/")?;

    // This allows us to take control of panic!() so we can send it to a file via the logger.
    panic::set_hook(Box::new(|panic_info| {
        let bt = Backtrace::new();

        error!("PANIC: {panic_info}, BACKTRACE: {bt:?}");
    }));

    // Starts an event gathering type for the window.
    let event_loop = EventLoop::new()?;

    let mut runner = Runner::Loading;
    Ok(event_loop.run_app(&mut runner)?)
}
