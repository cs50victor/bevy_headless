use bevy::{
    asset::Assets,
    ecs::{
        event::Event,
        system::{Commands, ResMut, Resource},
    },
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        texture::Image,
    },
};
use std::ops::Deref;

use image::{EncodableLayout, ImageBuffer, Pixel, Rgba, RgbaImage};

use crate::{ImageExportBundle, ImageExportSource};

#[derive(Default, Resource)]
pub struct CurrImage {
    pub img_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub frame_id: u64,
    pub extension: String,
}

impl CurrImage {
    pub fn update_data<P, Container>(
        &mut self,
        frame_id: u64,
        image_bytes: &ImageBuffer<P, Container>,
        extension: String,
    ) where
        P: Pixel + image::PixelWithColorType,
        [P::Subpixel]: EncodableLayout,
        Container: Deref<Target = [P::Subpixel]>,
    {
        self.frame_id = frame_id;

        self.extension = extension;

        let (w, h) = image_bytes.dimensions();
        if let Some(rgba_img_buff) = RgbaImage::from_raw(w, h, image_bytes.as_bytes().to_owned()) {
            self.img_buffer = rgba_img_buff;
        } else {
            log::error!("Error updating curr image image buffer");
        };
    }

    pub fn dimensions(&self) -> [u32; 2] {
        let (w, h) = self.img_buffer.dimensions();
        [w, h]
    }
}

#[derive(Debug, Default, Resource, Event)]
pub struct SceneInfo {
    width: u32,
    height: u32,
}

impl SceneInfo {
    pub fn new(width: u32, height: u32) -> SceneInfo {
        SceneInfo { width, height }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

pub fn setup_render_target(
    commands: &mut Commands,
    images: &mut ResMut<Assets<Image>>,
    scene_controller: &mut ResMut<SceneInfo>,
    mut export_sources: ResMut<Assets<ImageExportSource>>,
) -> RenderTarget {
    let size = Extent3d {
        width: scene_controller.width,
        height: scene_controller.height,
        ..Default::default()
    };

    // This is the texture that will be rendered to.
    let mut render_target_image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::COPY_SRC
                | TextureUsages::COPY_DST
                | TextureUsages::TEXTURE_BINDING
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..Default::default()
    };
    render_target_image.resize(size);
    let render_target_image_handle = images.add(render_target_image);

    commands.spawn(ImageExportBundle {
        source: export_sources.add(render_target_image_handle.clone().into()),
        ..Default::default()
    });

    RenderTarget::Image(render_target_image_handle)
}
