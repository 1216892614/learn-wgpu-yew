use gloo::net::http::Request;
use image::RgbaImage;

#[derive(Debug)]
pub struct Texture {
    pub(crate) texture: wgpu::Texture,
    pub(crate) view: wgpu::TextureView,
    pub(crate) sampler: wgpu::Sampler,
}

impl Texture {
    pub(crate) fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture_img: TextureImage,
        label: Option<&str>,
    ) -> Result<Self, anyhow::Error> {
        let dimensions = texture_img.dimensions();
        let rgba = texture_img.into_diffuse_rgba();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * dimensions.0),
                rows_per_image: std::num::NonZeroU32::new(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Ok(Self {
            texture,
            view,
            sampler,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TextureImage {
    diffuse_rgba: RgbaImage,
    /// (width, height) of image texture
    dimensions: (u32, u32),
}

impl TextureImage {
    pub(crate) async fn from_url(url: &str) -> Result<Self, anyhow::Error> {
        let resp = Request::get(url)
            .header("responseType", "blob")
            .send()
            .await?
            .binary()
            .await?;

        Ok(Self::from_bytes(&resp))
    }

    fn from_bytes(diffuse_bytes: &[u8]) -> Self {
        let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();
        let diffuse_rgba = diffuse_image.to_rgba8();

        use image::GenericImageView;
        let dimensions = diffuse_image.dimensions();

        Self {
            diffuse_rgba,
            dimensions,
        }
    }

    pub(crate) fn into_diffuse_rgba(self) -> RgbaImage {
        self.diffuse_rgba
    }

    pub(crate) fn dimensions(&self) -> (u32, u32) {
        self.dimensions
    }
}
