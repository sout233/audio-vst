use std::mem::transmute;
use std::os::raw::c_void;
use std::os::windows::io::HandleOrNull;
use std::path::{self, Path};
use std::ptr::null;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::{env, thread};

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use vst::buffer::AudioBuffer;
use vst::host::{Host, PluginInstance, PluginLoader};
use vst::plugin::Plugin;
use winit::dpi::LogicalSize;
use winit::event::Event;
use winit::event_loop::{self, EventLoop, EventLoopProxy};
use winit::window::WindowBuilder;

use crate::wav_decoder;

struct HostHandle;

const SAMPLE_RATE: usize = 48000;
const BLOCK_SIZE: usize = SAMPLE_RATE / 100;

impl Host for HostHandle {
    fn automate(&self, index: i32, value: f32) {
        println!("Parameter {} had its value changed to {}", index, value);
    }
    fn begin_edit(&self, index: i32) {
        println!("update_display")
    }
}

pub struct Box {
    pub host: Arc<Mutex<HostHandle>>,
    pub plugin: PluginInstance,
    pub loader: PluginLoader<HostHandle>,
}

impl Box {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        let path_binding = "E:\\VST\\Voxengo\\SPAN.dll".to_string();
        let path = Path::new(args.get(1).unwrap_or(&path_binding));

        let host: Arc<Mutex<HostHandle>> = Arc::new(Mutex::new(HostHandle));

        println!("Loading {}...", path.to_str().unwrap());

        // Load the plugin
        let mut loader = PluginLoader::load(path, Arc::clone(&host))
            .unwrap_or_else(|e| panic!("Failed to load plugin: {}", e));

        // Create an instance of the plugin
        let mut plugin = loader.instance().unwrap();

        // Get the plugin information
        let info = plugin.get_info();

        println!(
            "Loaded '{}':\n\t\
            Vendor: {}\n\t\
            Presets: {}\n\t\
            Parameters: {}\n\t\
            VST ID: {}\n\t\
            Version: {}\n\t\
            Initial Delay: {} samples",
            info.name,
            info.vendor,
            info.presets,
            info.parameters,
            info.unique_id,
            info.version,
            info.initial_delay
        );

        Box {
            host,
            plugin,
            loader,
        }
    }

    pub fn init(&mut self, sample_rate: f32, block_size: i64) {
        let plugin = &mut self.plugin;
        plugin.init();

        plugin.set_sample_rate(sample_rate);
        plugin.set_block_size(block_size);

        plugin.resume();

        println!("Initialized instance!");
    }

    pub fn show_editor(&mut self,event_loop: EventLoop<(Vec<f32>, Vec<f32>)> ) {
        let plugin = &mut self.plugin;

        let mut editor_view = plugin.get_editor().unwrap();

        let (window_width, window_height) = editor_view.size();

        let plugin_name = plugin.get_info().name.clone();

        let window = WindowBuilder::new()
            .with_inner_size(LogicalSize::new(window_width, window_height))
            .with_resizable(false)
            .with_title("sout VST testing host - ".to_owned() + &plugin_name)
            .build(&event_loop)
            .unwrap();

        let handle = window.raw_window_handle();

        let handle_ptr = match handle {
            Ok(RawWindowHandle::Win32(handle)) => handle.hwnd,
            _ => panic!("don't know this platform"),
        };

        unsafe {
            // let mut handle = 0x000C068C as *mut c_void;
            editor_view.open(transmute(handle_ptr));
        }

        let _ = event_loop.run(move |event, _| {
            match event {
                Event::NewEvents(_) => {}
                Event::WindowEvent { window_id, event } => {
                    // println!("WindowEvent({:?}): {:?}", window_id, event);
                }
                Event::DeviceEvent { device_id, event } => (),
                Event::UserEvent(sample) => {
                    static EMPTY_INPUTS: [f32; BLOCK_SIZE] = [0f32; BLOCK_SIZE];

                    let (left_samples, right_samples) = sample;

                    let inputs = [
                        left_samples.as_ptr(),
                        right_samples.as_ptr(),
                        EMPTY_INPUTS.as_ptr(),
                        EMPTY_INPUTS.as_ptr(),
                        EMPTY_INPUTS.as_ptr(),
                        EMPTY_INPUTS.as_ptr(),
                        EMPTY_INPUTS.as_ptr(),
                        EMPTY_INPUTS.as_ptr(),
                    ];

                    let mut output_buffers = [
                        vec![0f32; BLOCK_SIZE],
                        vec![0f32; BLOCK_SIZE],
                        vec![0f32; BLOCK_SIZE],
                        vec![0f32; BLOCK_SIZE],
                        vec![0f32; BLOCK_SIZE],
                        vec![0f32; BLOCK_SIZE],
                        vec![0f32; BLOCK_SIZE],
                        vec![0f32; BLOCK_SIZE],
                    ];

                    let mut outputs = output_buffers
                        .iter_mut()
                        .map(|buff| buff.as_mut_ptr())
                        .collect::<Vec<_>>();

                    let mut audio_buffer = unsafe {
                        AudioBuffer::from_raw(
                            inputs.len(),
                            outputs.len(),
                            inputs.as_ptr(),
                            outputs.as_mut_ptr(),
                            BLOCK_SIZE,
                        )
                    };

                    plugin.process(&mut audio_buffer);
                }
                Event::Suspended => (),
                Event::Resumed => (),
                Event::AboutToWait => (),
                Event::LoopExiting => (),
                Event::MemoryWarning => (),
            }
        });
    }
}
