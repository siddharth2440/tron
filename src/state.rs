use std::sync::Arc;
use cgmath::{InnerSpace, Matrix4, Point3, Vector3};
use wgpu::{util::DeviceExt, Surface};
use winit::{event_loop::OwnedDisplayHandle, window::Window};

use crate::{coords::CUBE_VERTICES, cube::Cube, texture};

#[derive(Debug)]
pub struct State {
    window: Arc<Window>,

    instance: wgpu::Instance,

    device: wgpu::Device,
    queue: wgpu::Queue,

    config: wgpu::wgt::SurfaceConfiguration<Vec<wgpu::TextureFormat>>,

    surface: Surface<'static>,
    
    texture_format: wgpu::TextureFormat,
    depth_texture: texture::Texture,

    vertex_buffer: wgpu::Buffer,

    render_pipeline: wgpu::RenderPipeline,

    win_height: u32,
    win_width: u32,

    cubes: Vec<Cube>
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelUniform {
    pub model_mat: [[f32; 4]; 4],
    pub view_matrix: [[f32; 4]; 4],
    pub projection_matrix: [[f32; 4]; 4]
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub vertex: [f32; 3],
    pub texture_coords: [f32; 2],
}


impl Vertex {

    fn desc() -> wgpu::VertexBufferLayout<'static> {

        wgpu::VertexBufferLayout { 
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress, 
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                
                // `position`
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    shader_location: 0,
                    offset: 0,
                },

                // `texture`
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    shader_location: 1,
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                }
            ]
        }
    }

}

impl State {
    pub async fn new(display: OwnedDisplayHandle, window: Arc<Window>) -> Result<State, ()> {
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
                    debug_fns: wgpu::GlDebugFns::default(),
                },
                noop: wgpu::NoopBackendOptions {
                    enable: false,
                    limits: Some(wgpu::Limits::defaults()),
                    features: Some(wgpu::Features::empty()),
                    device_type: Some(wgpu::DeviceType::Other),

                    subgroup_min_size: Some(1),
                    subgroup_max_size: Some(1),
                },

                dx12: wgpu::Dx12BackendOptions {
                    shader_compiler: wgpu::Dx12Compiler::Auto,
                    presentation_system: wgpu::Dx12SwapchainKind::DxgiFromVisual,
                    latency_waitable_object: wgpu::Dx12UseFrameLatencyWaitableObject::default(),
                    force_shader_model: wgpu::ForceShaderModelToken::default(),
                    agility_sdk: None,
                },
            },
            display: Some(Box::new(display)),
        };

        let instance: wgpu::Instance = wgpu::Instance::new(instance_descriptor);
        let surface: Surface<'_> = instance.create_surface(window.clone()).map_err(|err| {
            eprintln!("Error: create surface: {err}");
        })?;

        println!("Surface created. {surface:?}");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
                apply_limit_buckets: false,
            })
            .await
            .map_err(|err| {
                eprintln!("Error: Request adaptor Error: {err}");
            })?;

        let (device, queue) = match adapter
            .request_device(&wgpu::DeviceDescriptor {
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                label: Some("Request Device"),
                memory_hints: wgpu::MemoryHints::MemoryUsage,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits {
                    ..Default::default()
                },
                trace: wgpu::Trace::default(),
            })
            .await
        {
            Ok((d, q)) => (d, q),
            Err(err) => {
                println!("Error: request device err: {err}");
                return Err(());
            }
        };

        println!("Device Created: {device:?}");
        println!("Got Queue: {queue:?}");

        let surface_cap = surface.get_capabilities(&adapter);
        println!("INFO: SurfaceCapabilities {surface_cap:?}");

        let texture_format = surface_cap
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_cap.formats[0]);

        let physical_size = window.inner_size();

        let texture_bind_group_layout: wgpu::BindGroupLayout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { 
            label: Some("Texture Bind Group Layout"), 
            entries: &[
                
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    count: None,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture { 
                        sample_type: wgpu::TextureSampleType::Float { filterable: true }, 
                        view_dimension: wgpu::TextureViewDimension::D2, 
                        multisampled: false, 
                    },
                },

                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    count: None,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                },

            ]
        });

        let model_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { 
            label: Some("Model Bind Group Layout"), 
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    count: None,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer { 
                        ty: wgpu::BufferBindingType::Uniform, 
                        has_dynamic_offset: false, 
                        min_binding_size: None 
                    },
                }
            ]
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer Init Descriptor"),
            usage: wgpu::BufferUsages::VERTEX,
            contents: bytemuck::cast_slice(CUBE_VERTICES),
        });

        let surface_configuration: wgpu::SurfaceConfiguration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: texture_format,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: physical_size.width,
            height: physical_size.height,
            color_space: wgpu::SurfaceColorSpace::Srgb,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,

            view_formats: vec![],
        };

        let depth_texture = texture::Texture::create_depth_buffer(&surface_configuration, &device);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let pipeline_layout_desc: wgpu::PipelineLayoutDescriptor = wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout Descriptor"),
            immediate_size: 0,
            bind_group_layouts: &[
                Some(&texture_bind_group_layout),   // @group(0)
                Some(&model_bind_group_layout),     // @group(1)
            ],
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
                buffers: &[
                    Some(Vertex::desc())
                ],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                conservative: false,
                cull_mode: None,
                front_face: wgpu::FrontFace::Cw,
                polygon_mode: wgpu::PolygonMode::Fill,
                strip_index_format: None,
                unclipped_depth: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState { 
                format: texture::Texture::DEPTH_FORMAT, 
                depth_write_enabled: Some(true), 
                depth_compare: Some(wgpu::CompareFunction::Less), 
                stencil: wgpu::StencilState{
                    ..Default::default()
                }, 
                bias: wgpu::DepthBiasState { 
                    ..Default::default()
                }
            }),
            multisample: wgpu::MultisampleState {
                ..Default::default()
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: wgpu::PipelineCompilationOptions {
                    ..Default::default()
                },
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_configuration.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview_mask: None,
            cache: None,
        };
        let render_pipeline: wgpu::RenderPipeline = device.create_render_pipeline(&render_pipeline_desc);

        let texture1 = include_bytes!("../assets/a1.jpg");
        let texture2 = include_bytes!("../assets/a2.jpg");
        let texture3 = include_bytes!("../assets/a3.jpg");
        let texture4 = include_bytes!("../assets/a4.jpg");
        let texture5 = include_bytes!("../assets/a5.jpg");

        let textures: &[&[u8]] = &[
            texture1, texture2, texture3,
            texture4, texture5
        ];

        let cube_count: usize = 30;

        let mut positions: Vec<Vector3<f32>> = Vec::new();
        const MIN_DISTANCE: f32 = 3.0;

        while positions.len() < cube_count {

            let new_position = Vector3::new(
                rand::random_range(-10.0..10.0),
                rand::random_range(-5.0..5.0),
                rand::random_range(-10.0..-3.0),
            );

            let too_close = positions.iter().any(| existing_pos | {
                let distance = ( new_position - *existing_pos ).magnitude();
                distance < MIN_DISTANCE
            });

            if !too_close {
                positions.push(new_position);
            }
        }

        let cubes = positions.iter().map(| position: &Vector3<f32>| {
            let image_data =  textures[rand::random_range(0..textures.len())];
            Cube::new(
                *position, &device, &queue, &model_bind_group_layout, &texture_bind_group_layout, image_data
            )
        }).collect::<Vec<Cube>>();

        let mut state = State {
            window,
            instance,
            device,
            queue,
            surface: surface,
            texture_format: texture_format,
            depth_texture,

            vertex_buffer,

            win_height: physical_size.height as u32,
            win_width: physical_size.width as u32,
            render_pipeline,
            config: surface_configuration,

            cubes

        };

        state.reconfigure_state();

        return Ok(state);
    }

    pub fn reconfigure_state(&mut self) {
        let surface_configuration: wgpu::SurfaceConfiguration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.texture_format,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: self.win_width,
            height: self.win_height,
            color_space: wgpu::SurfaceColorSpace::Srgb,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,

            view_formats: vec![],
        };

        self.config = surface_configuration;
        self.surface.configure(&self.device, &self.config);
        self.depth_texture = texture::Texture::create_depth_buffer(&self.config, &self.device);
    }

    fn get_view_projection(&self) -> (Matrix4<f32>, Matrix4<f32>) {

        let view = Matrix4::look_at_rh( 
            Point3::new(0.0, 0.0, 3.0),
            Point3::new(0.0, 0.0, 0.0),
            Vector3::unit_y()
        );
        
        let aspect = self.win_width as f32 / self.win_height as f32;
        let projection = cgmath::perspective(cgmath::Deg(45.0), aspect, 0.1, 100.0);

        (view, projection)

    }

    pub fn render(&mut self) {

        self.window.request_redraw();

        let surface_texture = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Lost => {
                self.surface = self.instance.create_surface(self.window.clone()).unwrap();
                self.reconfigure_state();
                return;
            }
            wgpu::CurrentSurfaceTexture::Success(surface_tex) => surface_tex,
            wgpu::CurrentSurfaceTexture::Occluded | wgpu::CurrentSurfaceTexture::Timeout => {
                return;
            }
            wgpu::CurrentSurfaceTexture::Validation => {
                unreachable!("Error: validation error should be panic.");
            }
            wgpu::CurrentSurfaceTexture::Suboptimal(texture) => {
                drop(texture);
                self.reconfigure_state();
                return;
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.reconfigure_state();
                return;
            }
        };

        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                label: Some("texture view descriptor"),
                format: Some(self.texture_format.add_srgb_suffix()),
                ..Default::default()
            });

        let mut encoder: wgpu::CommandEncoder =
            self.device
                .create_command_encoder(&wgpu::wgt::CommandEncoderDescriptor {
                    label: Some("Encoder"),
                });

        let mut renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear( wgpu::Color::BLACK ),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { 
                view: &self.depth_texture.texture_view, 
                depth_ops: Some(wgpu::Operations { 
                    load: wgpu::LoadOp::Clear(1.0), 
                    store: wgpu::StoreOp::Store
                }), 
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        renderpass.set_pipeline(&self.render_pipeline);

        renderpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        let (view, projection) = self.get_view_projection();


        for cube in self.cubes.iter_mut() {

            cube.update();
            cube.update_uniform(&self.queue, view, projection);
            
            renderpass.set_bind_group(0, &cube.texture_bindgroup, &[]);
            renderpass.set_bind_group(1, &cube.bindgroup, &[]);
            renderpass.draw(0..36, 0..1);

        }

        drop(renderpass);

        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        self.queue.present(surface_texture);
    }
}
