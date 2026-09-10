use std::sync::Arc;
use wgpu::Surface;
use winit::{event_loop::OwnedDisplayHandle, window::Window};

#[derive(Debug)]
pub struct State {
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

    async fn new( display: OwnedDisplayHandle, window: Arc<Window> ) -> Result<State, ()> {

        let instance_descriptor: wgpu::InstanceDescriptor = wgpu::InstanceDescriptor { 
            backends: wgpu::Backends::GL, 
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

        
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptionsBase { 
            power_preference: wgpu::PowerPreference::LowPower, 
            force_fallback_adapter: true, 
            compatible_surface: Some(&surface), 
            apply_limit_buckets: true 
        }).await.map_err( |err| {
            eprintln!("Error: Request adaptor Error: {err}");
        })?;
        
        let ( device, queue) = match adapter.request_device(&wgpu::DeviceDescriptor {
            ..Default::default()
        }).await {
            Ok((d, q)) => (d, q),
            Err( err ) => {
                println!("Error: request device err: {err}");
                return Err(());
            }
        };
        
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
                entry_point: Some("vs_main"), 
                compilation_options: wgpu::PipelineCompilationOptions {
                    ..Default::default() 
                },
                buffers: &[None]
            }, 
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                conservative: true,
                cull_mode: Some(wgpu::Face::Back),
                front_face: wgpu::FrontFace::Ccw,
                polygon_mode: wgpu::PolygonMode::Line,
                strip_index_format: Some(wgpu::IndexFormat::Uint32),
                unclipped_depth: true
            },
            depth_stencil: Some(wgpu::DepthStencilState { 
                format: wgpu::TextureFormat::Depth32Float, 
                depth_write_enabled: Some(true), 
                depth_compare: Some(wgpu::CompareFunction::default()),
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(), 
            }), 
            multisample: wgpu::MultisampleState {
                ..Default::default()
            },
            fragment: None, 
            multiview_mask: None, 
            cache: None
        };
        let render_pipeline = device.create_render_pipeline(&render_pipeline_desc);

        return Ok(
            State { 
                instance,
                device, queue,
                surface: surface,
                texture_format: texture_format,
                win_height: physical_size.height as u32, 
                win_width: physical_size.width as u32, 
                render_pipeline,
                config: surface_configuration,
            }
        );
    }

    fn reconfigure_state( &mut self ) {
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

    fn render(&self) {
    }

}