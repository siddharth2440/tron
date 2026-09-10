use std::sync::Arc;
use wgpu::Surface;
use winit::{event_loop::OwnedDisplayHandle, window::Window};

#[derive(Debug)]
pub struct State {

    window: Arc<Window>,

    instance: wgpu::Instance,

    device: wgpu::Device,
    queue: wgpu::Queue,

    config: wgpu::wgt::SurfaceConfiguration<Vec<wgpu::TextureFormat>>,
    
    surface: Surface<'static>,
    texture_format: wgpu::TextureFormat,

    render_pipeline: wgpu::RenderPipeline,

    win_height: u32,
    win_width: u32
}

impl State {

    pub async fn new( display: OwnedDisplayHandle, window: Arc<Window> ) -> Result<State, ()> {

        let instance_descriptor: wgpu::InstanceDescriptor = wgpu::InstanceDescriptor { 
            backends: wgpu::Backends::all(), 
            flags: wgpu::InstanceFlags::default(), 
            memory_budget_thresholds: wgpu::MemoryBudgetThresholds { 
                for_resource_creation: Some(90),
                for_device_loss: Some(90), 
            },
            backend_options: wgpu::BackendOptions {
                gl: wgpu::GlBackendOptions { 
                    gles_minor_version: wgpu::Gles3MinorVersion::default(), 
                    fence_behavior: wgpu::GlFenceBehavior::default(), 
                    debug_fns: wgpu::GlDebugFns::default()
                },
                noop: wgpu::NoopBackendOptions { 
                    enable: false, 
                    limits: Some(wgpu::Limits::defaults()), 
                    features: Some(wgpu::Features::empty()),
                    device_type: Some(wgpu::DeviceType::Other), 
                    
                    subgroup_min_size: Some(1), 
                    subgroup_max_size: Some(1) 
                },

                dx12: wgpu::Dx12BackendOptions { 
                    shader_compiler: wgpu::Dx12Compiler::Auto, 
                    presentation_system: wgpu::Dx12SwapchainKind::DxgiFromVisual,
                    latency_waitable_object: wgpu::Dx12UseFrameLatencyWaitableObject::default(),
                    force_shader_model: wgpu::ForceShaderModelToken::default(),
                    agility_sdk: None,
                },
            },
            display: Some(Box::new(display))
        };

        let instance = wgpu::Instance::new(instance_descriptor);
        let surface = instance.create_surface(window.clone()).map_err(|err| {
            eprintln!("Error: create surface: {err}");
        })?;

        println!("Surface created. {surface:?}");

        
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptionsBase { 
            power_preference: wgpu::PowerPreference::default(), 
            force_fallback_adapter: false, 
            compatible_surface: Some(&surface), 
            apply_limit_buckets: false 
        }).await.map_err( |err| {
            eprintln!("Error: Request adaptor Error: {err}");
        })?;
        
        let ( device, queue) = match adapter.request_device(&wgpu::DeviceDescriptor {
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
            label: Some("Request Device"),
            memory_hints: wgpu::MemoryHints::MemoryUsage,
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits {
                ..Default::default()
            },
            trace: wgpu::Trace::default(),
        }).await {
            Ok((d, q)) => (d, q),
            Err( err ) => {
                println!("Error: request device err: {err}");
                return Err(());
            }
        };

        println!("Device Created: {device:?}");
        println!("Got Queue: {queue:?}");
        
        let surface_cap = surface.get_capabilities(&adapter);
        println!("INFO: SurfaceCapabilities {surface_cap:?}");

        let texture_format = surface_cap.formats.iter().find(| f | {
            f.is_srgb()
        }).copied().
        unwrap_or( surface_cap.formats[0] );

        let physical_size = window.inner_size();

        let surface_configuration: wgpu::SurfaceConfiguration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: texture_format,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width:  physical_size.width,
            height: physical_size.height,
            color_space: wgpu::SurfaceColorSpace::Srgb,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
            
            view_formats: vec![]
        };

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { 
            label: Some("shader"), 
            source: wgpu::ShaderSource::Wgsl( include_str!("shader.wgsl").into() )
        });

        let pipeline_layout_desc: wgpu::PipelineLayoutDescriptor = wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout Descriptor"),
            bind_group_layouts: &[],
            immediate_size: 0,
        };

        let pipeline_layout = device.create_pipeline_layout(&pipeline_layout_desc);

        let render_pipeline_desc: wgpu::RenderPipelineDescriptor = wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline Descriptor"), 
            layout: Some(&pipeline_layout), 
            vertex: wgpu::VertexState { 
                module: &shader, 
                entry_point: None, 
                compilation_options: wgpu::PipelineCompilationOptions {
                    ..Default::default() 
                },
                buffers: &[None]
            }, 
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                conservative: false,
                cull_mode: Some(wgpu::Face::Back),
                front_face: wgpu::FrontFace::Ccw,
                polygon_mode: wgpu::PolygonMode::Fill,
                strip_index_format: None,
                unclipped_depth: false
            },
            depth_stencil: None, 
            multisample: wgpu::MultisampleState {
                ..Default::default()
            },
            fragment: Some(wgpu::FragmentState { 
                module: &shader, 
                entry_point: Some("fs_main"), 
                compilation_options: wgpu::PipelineCompilationOptions { 
                    ..Default::default() 
                },
                targets: &[
                    Some(wgpu::ColorTargetState { 
                        format: surface_configuration.format, 
                        blend: Some(wgpu::BlendState::REPLACE), 
                        write_mask: wgpu::ColorWrites::ALL,
                    })
                ],
            }),
            multiview_mask: None, 
            cache: None
        };
        let render_pipeline = device.create_render_pipeline(&render_pipeline_desc);

        
        let mut state =  State { 
            window,
            instance,
            device, queue,
            surface: surface,
            texture_format: texture_format,
            win_height: physical_size.height as u32, 
            win_width: physical_size.width as u32, 
            render_pipeline,
            config: surface_configuration,
        };

        state.reconfigure_state();

        return Ok(state);
    }

    pub fn reconfigure_state( &mut self ) {
        let surface_configuration: wgpu::SurfaceConfiguration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.texture_format,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width:  self.win_width,
            height: self.win_height,
            color_space: wgpu::SurfaceColorSpace::Srgb,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
            
            view_formats: vec![]
        };

        self.config = surface_configuration;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn render(&mut self) {
        self.window.request_redraw();

        let surface_texture = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Lost => {
                self.surface = self.instance.create_surface(self.window.clone()).unwrap();
                self.reconfigure_state();
                return;
            },
            wgpu::CurrentSurfaceTexture::Success( surface_tex ) => surface_tex,
            wgpu::CurrentSurfaceTexture::Occluded | wgpu::CurrentSurfaceTexture::Timeout => {
                return;
            },
            wgpu::CurrentSurfaceTexture::Validation => {
                unreachable!("Error: validation error should be panic.");
            },
            wgpu::CurrentSurfaceTexture::Suboptimal(texture) => {
                drop(texture);
                self.reconfigure_state();
                return;
            },
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.reconfigure_state();
                return;
            }
        };

        let texture_view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("texture view descriptor"),
            format: Some(self.texture_format.add_srgb_suffix()),
            ..Default::default()
        });

        let mut encoder: wgpu::CommandEncoder = self.device.create_command_encoder(&wgpu::wgt::CommandEncoderDescriptor {
            label: Some("Encoder") 
        });

        let mut renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { 
            label: Some("Render Pass"), 
            color_attachments: &[Some(wgpu::RenderPassColorAttachment { 
                view: &texture_view, 
                depth_slice: None, 
                resolve_target: None, 
                ops: wgpu::Operations { 
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLUE), 
                    store: wgpu::StoreOp::Store,
                },
            })], 
            depth_stencil_attachment: None,
            timestamp_writes: None, 
            occlusion_query_set: None, 
            multiview_mask: None
        });

        renderpass.set_pipeline(&self.render_pipeline);
        renderpass.draw(0..3, 0..1);

        drop(renderpass);

        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        self.queue.present(surface_texture);

    }

}