#![allow(dead_code, clippy::collapsible_match, unused_imports)]
use backtrace::Backtrace;
use camera::{
    controls::{Controls, FlatControls, FlatSettings},
    Projection,
};
use cosmic_text::{Attrs, Metrics};
use graphics::{
    wgpu::{BackendOptions, Dx12BackendOptions, MemoryBudgetThresholds, NoopBackendOptions},
    *,
};
use input::{Bindings, FrameTime, InputHandler, Key};
use log::{error, info, warn, Level, LevelFilter, Metadata, Record};
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{self, File},
    io::{prelude::*, Read, Write},
    iter, panic,
    sync::Arc,
    time::{Duration, Instant},
};
use wgpu::{Backends, Dx12Compiler, InstanceDescriptor, InstanceFlags};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::*,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    platform::windows::WindowAttributesExtWindows,
    window::{WindowAttributes, WindowButtons},
};

mod collection;
mod config;
mod editor_input;
mod gfx_collection;
mod interface;
mod map;
mod map_data;
mod renderer;
mod resource;
mod tileset;

use collection::*;
use config::*;
use editor_input::{dialog_input::*, *};
use gfx_collection::*;
use interface::*;
use map::*;
use map_data::*;
use renderer::*;
use resource::*;
use tileset::*;

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
        config_data: ConfigData,
        systems: DrawSetting,
        graphics: Graphics<FlatControls>,
        gui: Interface,
        tileset: Tileset,
        gameinput: GameInput,
        database: EditorData,
        mapview: MapView,
        input_handler: InputHandler<Action, Axis>,
        frame_time: FrameTime,
        time: f32,
        fps: u32,
        mouse_pos: PhysicalPosition<f64>,
        mouse_press: bool,
    },
}

impl winit::application::ApplicationHandler for Runner {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Self::Loading = self {
            info!("loading initiation");
            let win_attrs = WindowAttributes::default()
                .with_active(false)
                .with_visible(false)
                .with_inner_size(PhysicalSize::new(949.0 * ZOOM_LEVEL, 802.0 * ZOOM_LEVEL))
                .with_title("Map Editor")
                .with_enabled_buttons({
                    let mut buttons = WindowButtons::all();
                    buttons.remove(WindowButtons::MAXIMIZE);
                    buttons
                });

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
                    dx12: Dx12BackendOptions {
                        shader_compiler: Dx12Compiler::default(),
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
                },
                // How we are presenting the screen which causes it to either clip to a FPS limit or be unlimited.
                wgpu::PresentMode::AutoVsync,
            ))
            .unwrap();

            info!("after renderer initiation");
            // we print the GPU it decided to use here for testing purposes.
            println!("{:?}", renderer.adapter().get_info());

            // We generate Texture atlases to use with out types.
            let mut atlases: Vec<AtlasSet> = iter::from_fn(|| {
                Some(AtlasSet::new(
                    &mut renderer,
                    wgpu::TextureFormat::Rgba8UnormSrgb,
                    true,
                    2048,
                ))
            })
            .take(4)
            .collect();

            // we generate the Text atlas seperatly since it contains a special texture that only has the red color to it.
            // and another for emojicons.
            let text_atlas = TextAtlas::new(&mut renderer, 1024).unwrap();

            // get the screen size.
            let size = renderer.size();
            let mat = Mat4::from_translation(Vec3 {
                x: 40.0,
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
            let resource = TextureAllocation::new(&mut atlases, &renderer).unwrap();

            // Compile all rendering data in one type for quick access and passing
            let mut systems = DrawSetting {
                gfx: GfxCollection::new(),
                renderer,
                size,
                scale,
                resource,
                audio_list: AudioCollection::new(),
            };

            // We establish the different renderers here to load their data up to use them.
            let text_renderer = TextRenderer::new(&systems.renderer).unwrap();
            let image_renderer = ImageRenderer::new(&systems.renderer).unwrap();
            let mut map_renderer = MapRenderer::new(&mut systems.renderer, 81).unwrap();
            let ui_renderer = RectRenderer::new(&systems.renderer).unwrap();

            // Initiate map editor data
            let mut config_data = load_config();
            let gui = Interface::new(&mut systems, &mut config_data);
            let tileset = Tileset::new(&mut systems, &mut map_renderer, &mut config_data);
            let gameinput = GameInput::new();
            let mut mapview = MapView::new(&mut systems, &mut map_renderer, &mut config_data);
            let mut database = EditorData::new().unwrap();

            // Load the initial map
            database.load_map_data(&mut systems, &mut mapview);
            database.load_link_maps(&mut mapview);

            // setup our system which includes Camera and projection as well as our controls.
            // for the camera.
            let system = System::new(
                &mut systems.renderer,
                Projection::Orthographic {
                    left: 0.0,
                    right: size.width,
                    bottom: 0.0,
                    top: size.height,
                    near: 1.0,
                    far: -100.0,
                },
                FlatControls::new(FlatSettings { zoom: ZOOM_LEVEL }),
                [size.width, size.height],
                mat,
                1.5,
            );

            // Allow the window to be seen. hiding it then making visible speeds up
            // load times.
            systems.renderer.window().set_visible(true);

            // add everything into our convience type for quicker access and passing.
            let graphics = Graphics {
                system,
                image_atlas: atlases.remove(0),
                map_renderer,
                map_atlas: atlases.remove(0),
                image_renderer,
                text_atlas,
                text_renderer,
                ui_renderer,
                ui_atlas: atlases.remove(0),
            };

            // Create the mouse/keyboard bindings for our stuff.
            let bindings = Bindings::<Action, Axis>::new();

            systems.renderer.window().set_visible(true);

            *self = Self::Ready {
                config_data,
                systems,
                graphics,
                gui,
                tileset,
                gameinput,
                database,
                mapview,
                input_handler: InputHandler::new(bindings, Duration::from_millis(180)),
                frame_time: FrameTime::new(),
                time: 0.0f32,
                fps: 0u32,
                mouse_pos: PhysicalPosition::new(0.0, 0.0),
                mouse_press: false,
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
            config_data,
            systems,
            graphics,
            gui,
            tileset,
            gameinput,
            database,
            mapview,
            input_handler,
            frame_time,
            time,
            fps,
            mouse_pos,
            mouse_press,
        } = self
        {
            if window_id == systems.renderer.window().id() {
                match &event {
                    WindowEvent::CloseRequested => {
                        // Close preference window
                        if gui.preference.is_open {
                            config_data.set_data(load_config());
                            gui.preference.close(systems);
                        }
                        if database.got_changes() {
                            // We found changes on our map, we need to confirm if we would like to proceed to exit the editor
                            gui.open_dialog(
                                systems,
                                DialogType::MapSave,
                                Some(database.did_map_change.clone()),
                            );
                        } else {
                            gui.open_dialog(systems, DialogType::ExitConfirm, None);
                        }
                    }
                    WindowEvent::KeyboardInput { event, .. } => {
                        if !handle_key_input(event, gui, mapview, database, systems) {
                            // Make sure that we only trigger the shortcut keys when we are not on a textbox
                            access_shortcut(
                                event,
                                systems,
                                gameinput,
                                database,
                                tileset,
                                mapview,
                                gui,
                                config_data,
                            );
                        };
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        *mouse_pos = *position;

                        if *mouse_press {
                            handle_input(
                                systems,
                                MouseInputType::LeftDownMove,
                                &Vec2::new(mouse_pos.x as f32, mouse_pos.y as f32),
                                gameinput,
                                gui,
                                tileset,
                                mapview,
                                database,
                                config_data,
                                event_loop,
                            );
                        } else {
                            handle_input(
                                systems,
                                MouseInputType::Move,
                                &Vec2::new(mouse_pos.x as f32, mouse_pos.y as f32),
                                gameinput,
                                gui,
                                tileset,
                                mapview,
                                database,
                                config_data,
                                event_loop,
                            );
                        }
                    }
                    WindowEvent::MouseInput { state, .. } => match state {
                        ElementState::Pressed => {
                            handle_input(
                                systems,
                                MouseInputType::LeftDown,
                                &Vec2::new(mouse_pos.x as f32, mouse_pos.y as f32),
                                gameinput,
                                gui,
                                tileset,
                                mapview,
                                database,
                                config_data,
                                event_loop,
                            );
                            *mouse_press = true;
                        }
                        ElementState::Released => {
                            handle_input(
                                systems,
                                MouseInputType::Release,
                                &Vec2::new(mouse_pos.x as f32, mouse_pos.y as f32),
                                gameinput,
                                gui,
                                tileset,
                                mapview,
                                database,
                                config_data,
                                event_loop,
                            );
                            *mouse_press = false;
                        }
                    },
                    _ => {}
                }
            }

            // update our renderer based on events here
            if !systems.renderer.update(&event).unwrap() {
                return;
            }

            // get the current window size so we can see if we need to resize the renderer.
            let new_size = systems.renderer.size();

            // update our inputs.
            input_handler.window_updates(&event);

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
            }

            frame_time.update();
            let seconds = frame_time.seconds();
            // update our systems data to the gpu. this is the Camera in the shaders.
            graphics.system.update(&systems.renderer, frame_time);

            // update our systems data to the gpu. this is the Screen in the shaders.
            graphics
                .system
                .update_screen(&systems.renderer, [new_size.width, new_size.height]);

            // This adds the Image data to the Buffer for rendering.
            add_image_to_buffer(systems, graphics, mapview, gui, tileset);

            // this cycles all the Image's in the Image buffer by first putting them in rendering order
            // and then uploading them to the GPU if they have moved or changed in any way. clears the
            // Image buffer for the next render pass. Image buffer only holds the ID's and Sortign info
            // of the finalized Indicies of each Image.
            graphics.image_renderer.finalize(&mut systems.renderer);
            graphics.map_renderer.finalize(&mut systems.renderer);
            graphics.text_renderer.finalize(&mut systems.renderer);
            graphics.ui_renderer.finalize(&mut systems.renderer);

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
                systems.gfx.set_text(
                    &mut systems.renderer,
                    gui.labels[LABEL_FPS],
                    &format!("FPS: {fps}"),
                );
                *fps = 0u32;
                *time = seconds + 1.0;
            }
            *fps += 1;

            systems.renderer.window().pre_present_notify();
            systems.renderer.present().unwrap();

            // These clear the Last used image tags.
            //Can be used later to auto unload things not used anymore if ram/gpu ram becomes a issue.
            if *fps == 1 {
                graphics.image_atlas.trim();
                graphics.map_atlas.trim();
                graphics.text_atlas.trim();
                systems.renderer.font_sys.shape_run_cache.trim(1024);
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
            config_data: _,
            systems: _,
            graphics: _,
            gui: _,
            tileset: _,
            gameinput: _,
            database: _,
            mapview: _,
            input_handler,
            frame_time: _,
            time: _,
            fps: _,
            mouse_pos: _,
            mouse_press: _,
        } = self
        {
            input_handler.device_updates(&event);
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Self::Ready {
            config_data: _,
            systems,
            graphics: _,
            gui: _,
            tileset: _,
            gameinput: _,
            database: _,
            mapview: _,
            input_handler: _,
            frame_time: _,
            time: _,
            fps: _,
            mouse_pos: _,
            mouse_press: _,
        } = self
        {
            systems.renderer.window().request_redraw();
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), GraphicsError> {
    // Create logger to output to a File
    log::set_logger(&MY_LOGGER).unwrap();
    // Set the Max level we accept logging to the file for.
    log::set_max_level(LevelFilter::Info);

    info!("starting up");

    // Create the directory for our map data
    fs::create_dir_all("./data/maps/")?;

    // This allows us to take control of panic!() so we can send it to a file via the logger.
    panic::set_hook(Box::new(|panic_info| {
        let bt = Backtrace::new();

        error!("PANIC: {panic_info}, BACKTRACE: {bt:?}");
    }));

    env::set_var("WGPU_VALIDATION", "0");
    env::set_var("WGPU_DEBUG", "0");
    // Starts an event gathering type for the window.
    let event_loop = EventLoop::new()?;

    let mut runner = Runner::Loading;
    Ok(event_loop.run_app(&mut runner)?)
}
