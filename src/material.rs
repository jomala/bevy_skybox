use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::{
    pipeline::{PipelineDescriptor, RenderPipeline},
    render_graph::{base, AssetRenderResourcesNode, RenderGraph},
    renderer::RenderResources,
    shader::{ShaderStage, ShaderStages},
};

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "a9ef25aa-cfa7-4990-81b9-cd02533ac3f1"]
pub struct SkyMaterial {
    pub texture: Handle<Texture>,
}

const SKY_VERTEX_SHADER: &str = include_str!("../assets/sky.vert");
const SKY_FRAGMENT_SHADER: &str = include_str!("../assets/sky.frag");

impl SkyMaterial {
    pub fn pipeline(
        mut pipelines: ResMut<Assets<PipelineDescriptor>>,
        mut shaders: ResMut<Assets<Shader>>,
        mut render_graph: ResMut<RenderGraph>,
    ) -> RenderPipelines {
        let mut descriptor = PipelineDescriptor::default_config(ShaderStages {
            vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, SKY_VERTEX_SHADER)),
            fragment: Some(shaders.add(Shader::from_glsl(
                ShaderStage::Fragment,
                SKY_FRAGMENT_SHADER,
            ))),
        });
        descriptor.depth_stencil =
            descriptor
                .depth_stencil
                .map(|mut depth_stencil_state| {
                    depth_stencil_state.depth_compare =
                        bevy::render::pipeline::CompareFunction::LessEqual;
                    depth_stencil_state.depth_write_enabled = false;
                    depth_stencil_state
                });

        let sky_pipeline_handle = pipelines.add(descriptor);
        render_graph.add_system_node(
            "SkyMaterial",
            AssetRenderResourcesNode::<SkyMaterial>::new(true),
        );
        render_graph
            .add_node_edge("SkyMaterial", base::node::MAIN_PASS)
            .unwrap();

        let render_pipelines =
            RenderPipelines::from_pipelines(vec![RenderPipeline::new(sky_pipeline_handle)]);
        render_pipelines
    }
}
