use image::GenericImageView;


#[derive(Debug)]
pub struct Texture {
    pub texture: wgpu::Texture,
    pub texture_view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}


impl Texture {

    pub fn from_bytes( device: &wgpu::Device, queue: &wgpu::Queue, image_data_in_bytes: &[u8] ) -> Result<Self, ()> {
        let image = image::load_from_memory(image_data_in_bytes).expect("Image load from memoery error.");
        Ok(Self::from_image(&device, &queue, &image))
    }

    pub fn from_image( device: &wgpu::Device, queue: &wgpu::Queue, img: &image::DynamicImage ) -> Texture {
        
        let data = img.to_rgba8();
        let ( width, height ) =img.dimensions();

        let tex_size = wgpu::Extent3d {
            width, height,
            depth_or_array_layers: 1,
        };

        let tex_desc: wgpu::TextureDescriptor = wgpu::wgt::TextureDescriptor { 
            label: Some("Texture Descriptor"), 
            size: tex_size, 
            mip_level_count: 1u32, 
            sample_count: 1, 
            dimension: wgpu::TextureDimension::D2, 
            format: wgpu::TextureFormat::Rgba8UnormSrgb, 
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, 
            view_formats: &[]
        };

        let texture = device.create_texture(&tex_desc);
        let texelcopytextureinfo = wgpu::TexelCopyTextureInfoBase { 
            texture: &texture, 
            mip_level: 0, 
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        };

        queue.write_texture(
            texelcopytextureinfo, 
            &data,
            wgpu::TexelCopyBufferLayout { 
                offset: 0,
                bytes_per_row:  Some( 4 * width ), 
                rows_per_image: Some( height ) 
            }, 
            tex_size
        );

        let texture_view = texture.create_view(&wgpu::wgt::TextureViewDescriptor { 
            label: Some("Texture View Descriptor"), 
            ..Default::default()
        });

        let sampler = device.create_sampler(&wgpu::wgt::SamplerDescriptor { 
            label: Some("Sampler"), 
            address_mode_u: wgpu::AddressMode::ClampToEdge, 
            address_mode_v: wgpu::AddressMode::ClampToEdge, 
            address_mode_w: wgpu::AddressMode::ClampToEdge, 
            mag_filter: wgpu::FilterMode::Linear, 
            min_filter: wgpu::FilterMode::Nearest, 
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        Self { texture, texture_view, sampler }

    }

    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn create_depth_buffer(surface_config: &wgpu::SurfaceConfiguration , device: &wgpu::Device) -> Self {

        let size: wgpu::Extent3d = wgpu::Extent3d { 
            width: surface_config.width,  
            height: surface_config.height, 
            depth_or_array_layers: 1 
        };

        let texture_desc = wgpu::TextureDescriptor{
            label: Some(" Texture Descriptor "),
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            size,
            mip_level_count: 1,
            sample_count: 1,
            view_formats: &[],
        };

        let texture = device.create_texture(&texture_desc);


        let texture_view_desc: wgpu::TextureViewDescriptor = wgpu::wgt::TextureViewDescriptor { 
            label: Some("Texture View Descriptor"), 
            ..Default::default()
        };
        let texture_view = texture.create_view(&texture_view_desc);

        let sampler_desc: wgpu::SamplerDescriptor = wgpu::wgt::SamplerDescriptor { 
            label: Some("Sampler Descriptor"), 
            address_mode_u: wgpu::AddressMode::ClampToEdge, 
            address_mode_v: wgpu::AddressMode::ClampToEdge, 
            address_mode_w: wgpu::AddressMode::ClampToEdge, 
            mag_filter: wgpu::FilterMode::Linear, 
            min_filter: wgpu::FilterMode::Linear, 
            mipmap_filter: wgpu::MipmapFilterMode::Nearest, 
            lod_min_clamp: 0.0, 
            lod_max_clamp: 100.0, 
            compare: Some(wgpu::CompareFunction::Less), 
            ..Default::default()
        };
        let sampler = device.create_sampler(&sampler_desc);

        Self { texture, texture_view, sampler }

    }

}