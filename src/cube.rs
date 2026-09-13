use cgmath::{Matrix4, Vector3};
use wgpu::util::DeviceExt;

use crate::{state::ModelUniform, texture};

#[derive(Debug)]
pub struct Cube {
    pub position: Vector3<f32>,

    pub rotation_x: f32,
    pub rotation_y: f32,
    pub rotation_z: f32,

    pub rotation_speed_x: f32,
    pub rotation_speed_y: f32,
    pub rotation_speed_z: f32,

    pub uniform_buffer: wgpu::Buffer,
    pub bindgroup: wgpu::BindGroup,

    pub texture: texture::Texture,
    pub texture_bindgroup: wgpu::BindGroup,
}


impl Cube {

    pub fn new(
        position: Vector3<f32>, device: &wgpu::Device, queue: &wgpu::Queue, bind_group_layout: &wgpu::BindGroupLayout,
        texture_bind_group_layout: &wgpu::BindGroupLayout, image_data_in_bytes: &[u8]
    ) -> Self {

        // texture

        let texture = texture::Texture::from_bytes(&device, &queue, image_data_in_bytes).expect("texture creation error");

        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor { 
            label: Some("Texture Bind Group."), 
            layout: &texture_bind_group_layout, 
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.texture_view)
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler)
                }
            ]
        });

        let uniform = ModelUniform {
            model_mat: Matrix4::from_translation(position).into(),
            projection_matrix: Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 
                0.0, 1.0, 0.0, 0.0, 
                0.0, 0.0, 1.0, 0.0, 
                0.0, 0.0, 0.0, 1.0
            ).into(),
            view_matrix: Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 
                0.0, 1.0, 0.0, 0.0, 
                0.0, 0.0, 1.0, 0.0, 
                0.0, 0.0, 0.0, 1.0
            ).into(),
        };

        let uniform_data = [uniform];

        let buffer_init_desc: wgpu::util::BufferInitDescriptor = wgpu::util::BufferInitDescriptor {
            label: Some("Cube uniform Buffer"),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            contents: bytemuck::cast_slice(&uniform_data)
        };
        
        let uniform_buffer = device.create_buffer_init(&buffer_init_desc);


        let bind_group_descriptor: wgpu::BindGroupDescriptor = wgpu::BindGroupDescriptor { 
            label: Some("Cube: Bind Group Descriptor"), 
            layout: bind_group_layout, 
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                }
            ] 
        };
        let bind_group = device.create_bind_group(&bind_group_descriptor);


        Self { 
            position, 
            rotation_x: 0.0, 
            rotation_y: 0.0, 
            rotation_z: 0.0, 

            rotation_speed_x: rand::random_range(0.005..0.02), 
            rotation_speed_y: rand::random_range(0.005..0.02), 
            rotation_speed_z: rand::random_range(0.005..0.02) ,

            bindgroup: bind_group,
            uniform_buffer,

            texture_bindgroup: texture_bind_group,
            texture
        }

    }

    pub fn update(&mut self) {

        self.rotation_x += self.rotation_speed_x;
        self.rotation_y += self.rotation_speed_y;
        self.rotation_z += self.rotation_speed_z;

    }

    pub fn get_model_matrix(&self) -> Matrix4<f32> {

        let translation = Matrix4::from_translation(self.position);

        let rotation_x: Matrix4<f32> = Matrix4::from_angle_x(cgmath::Rad(self.rotation_x));
        let rotation_y: Matrix4<f32> = Matrix4::from_angle_y(cgmath::Rad(self.rotation_y));
        let rotation_z: Matrix4<f32> = Matrix4::from_angle_z(cgmath::Rad(self.rotation_z));

        translation * rotation_x * rotation_y * rotation_z

    }


    pub fn update_uniform(&self, queue: &wgpu::Queue, view: Matrix4<f32>, projection: Matrix4<f32>) {

        let model = self.get_model_matrix();

        let uniform = ModelUniform {
            model_mat: model.into(),
            view_matrix: view.into(),
            projection_matrix: projection.into(),
        };

        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniform]));

    }

}