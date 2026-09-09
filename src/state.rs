use std::sync::Arc;

use winit::{event_loop::OwnedDisplayHandle, window::Window};

#[derive(Debug)]
pub struct State {
    instance: wgpu::Instance,

    // surface_configuration: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,

    win_height: u32,
    win_width: u32
}

impl State {

    async fn new( display: OwnedDisplayHandle, window: Arc<Window> ) -> Result<State, ()> {

        let instance_descriptor: wgpu::InstanceDescriptor = wgpu::InstanceDescriptor { 
            backends: wgpu::Backends::default(), 
            flags: wgpu::InstanceFlags::default(), 
            memory_budget_thresholds: wgpu::MemoryBudgetThresholds { 
                for_resource_creation: Some(90), 
                for_device_loss: Some(90), 
            },
            backend_options: wgpu::BackendOptions::default(),
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

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { 
            label: Some("shader"), 
            source: wgpu::ShaderSource::Wgsl( include_str!("shader.wgsl").into() )
        });

        let _surface_configuration: wgpu::SurfaceConfiguration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: texture_format,
            alpha_mode: surface_cap.alpha_modes[0],
            width: physical_size.width,
            height: physical_size.height,
            color_space: wgpu::SurfaceColorSpace::Srgb,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
            view_formats: vec![]
        };

        let pipeline_layout_desc: wgpu::PipelineLayoutDescriptor = wgpu::PipelineLayoutDescriptor {
            ..Default::default()
        };

        let pipeline_layout = device.create_pipeline_layout(&pipeline_layout_desc);

        let render_pipeline_desc: wgpu::RenderPipelineDescriptor = wgpu::RenderPipelineDescriptor { 
            label: Some("Render Pipeline Descriptor"), 
            layout: Some(&pipeline_layout), 
            vertex: wgpu::VertexState { 
                module: &shader, 
                entry_point: Some("vs_main"), 
                compilation_options: wgpu::PipelineCompilationOptions { ..Default::default() }, 
                buffers: &[None]
            }, 
            primitive: wgpu::PrimitiveState {
                ..Default::default()
            },
            depth_stencil: None, 
            multisample: wgpu::MultisampleState { 
                ..Default::default()
            }, 
            fragment: None, 
            multiview_mask: None, 
            cache: None
        };
        let render_pipeline = device.create_render_pipeline(&render_pipeline_desc);

        return Ok(State { instance, win_height: 0u32, win_width: 0u32, render_pipeline });
    }

}