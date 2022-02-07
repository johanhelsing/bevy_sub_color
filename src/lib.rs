use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    math::vec4,
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_asset::{PrepareAssetError, RenderAsset, RenderAssets},
        render_resource::{BlendComponent, BlendFactor, BlendOperation, BlendState},
        renderer::RenderDevice,
    },
    sprite::{GpuColorMaterial, Material2dPipeline, Material2dPlugin, SpecializedMaterial2d},
};
use derive_more::From;

pub struct SubColorMaterialPlugin;

impl Plugin for SubColorMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<SubColorMaterial>::default());
    }
}

#[derive(Debug, Clone, TypeUuid, From)]
#[uuid = "d8dec015-848e-4da6-beef-73cabec7bde6"]
pub struct SubColorMaterial(pub Color);

impl RenderAsset for SubColorMaterial {
    type ExtractedAsset = SubColorMaterial;
    type PreparedAsset = GpuColorMaterial;

    type Param = (
        SRes<RenderDevice>,
        SRes<Material2dPipeline<ColorMaterial>>,
        SRes<RenderAssets<Image>>,
    );

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        param: &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let color_material = ColorMaterial {
            color: (extracted_asset.0 * vec4(-1., -1., -1., 0.))
                + vec4(1., 1., 1., extracted_asset.0.a()),
            texture: Default::default(),
        };
        let color_prepare = ColorMaterial::prepare_asset(color_material, param);
        color_prepare.map_err(|_| PrepareAssetError::RetryNextUpdate(extracted_asset))
    }
}

impl SpecializedMaterial2d for SubColorMaterial {
    type Key = ();

    fn key(_material: &<Self as RenderAsset>::PreparedAsset) -> Self::Key {}

    fn specialize(
        _key: Self::Key,
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
    ) {
        let fragment = descriptor.fragment.as_mut().unwrap();
        fragment.targets[0].blend = Some(BlendState {
            color: BlendComponent {
                src_factor: BlendFactor::One,
                dst_factor: BlendFactor::One,
                operation: BlendOperation::ReverseSubtract,
            },
            alpha: BlendComponent::OVER,
        });
    }

    fn bind_group(
        render_asset: &<Self as RenderAsset>::PreparedAsset,
    ) -> &bevy::render::render_resource::BindGroup {
        ColorMaterial::bind_group(render_asset)
    }

    fn bind_group_layout(
        render_device: &bevy::render::renderer::RenderDevice,
    ) -> bevy::render::render_resource::BindGroupLayout {
        ColorMaterial::bind_group_layout(render_device)
    }

    fn vertex_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        ColorMaterial::vertex_shader(asset_server)
    }

    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        ColorMaterial::fragment_shader(asset_server)
    }
}