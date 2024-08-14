mod batch;
mod compositor;
mod image_cache;
pub mod text;
pub mod util;

use crate::components::core::orthographic_projection;
use crate::components::rich_text::image_cache::{GlyphCache, ImageCache};
use crate::context::Context;
use crate::font::FontLibraryData;
use crate::layout::SugarDimensions;
use compositor::{
    CachedRun, Command, Compositor, DisplayList, Rect, TextureEvent, TextureId, Vertex,
};
use rustc_hash::FxHashMap;
use std::{borrow::Cow, mem};
use text::{Glyph, TextRunStyle};
use wgpu::util::DeviceExt;
use wgpu::{Texture, PrimitiveState};

// Note: currently it's using Indexed drawing instead of Instance drawing could be worth to
// evaluate if would make sense move to instance drawing instead
// https://math.hws.edu/graphicsbook/c9/s2.html
// https://docs.rs/wgpu/latest/wgpu/enum.VertexStepMode.html

pub const BLEND: Option<wgpu::BlendState> = Some(wgpu::BlendState {
    color: wgpu::BlendComponent {
        src_factor: wgpu::BlendFactor::SrcAlpha,
        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
        operation: wgpu::BlendOperation::Add,
    },
    alpha: wgpu::BlendComponent {
        src_factor: wgpu::BlendFactor::One,
        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
        operation: wgpu::BlendOperation::Add,
    },
});

pub struct RichTextBrush {
    vertex_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    sampler: wgpu::Sampler,
    color_texture_view: wgpu::TextureView,
    mask_texture_view: wgpu::TextureView,
    transform: wgpu::Buffer,
    bind_group_layout: wgpu::BindGroupLayout,
    pipeline: wgpu::RenderPipeline,
    textures: FxHashMap<TextureId, Texture>,
    current_transform: [f32; 16],
    comp: Compositor,
    draw_layout_cache: DrawLayoutCache,
    dlist: DisplayList,
    bind_group_needs_update: bool,
    first_run: bool,
    images: ImageCache,
    glyphs: GlyphCache,
    vertices_quantity: usize,
}

impl RichTextBrush {
    pub fn new(context: &Context) -> Self {
        let device = &context.device;
        let dlist = DisplayList::new();
        let vertices_quantity = 1;

        let current_transform =
            orthographic_projection(context.size.width, context.size.height);
        let transform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&current_transform),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create pipeline layout
        let bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: wgpu::BufferSize::new(mem::size_of::<
                                [f32; 16],
                            >(
                            )
                                as wgpu::BufferAddress),
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::VERTEX
                            | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float {
                                filterable: true,
                            },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::VERTEX
                            | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float {
                                filterable: true,
                            },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 3,
                        visibility: wgpu::ShaderStages::VERTEX
                            | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(
                            wgpu::SamplerBindingType::Filtering,
                        ),
                        count: None,
                    },
                ],
            });

        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                push_constant_ranges: &[],
                bind_group_layouts: &[&bind_group_layout],
            });

        let color_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("rich_text create color_texture"),
            size: wgpu::Extent3d {
                width: context.size.width as u32,
                height: context.size.height as u32,
                depth_or_array_layers: 1,
            },
            view_formats: &[],
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
            mip_level_count: 1,
            sample_count: 1,
        });
        let color_texture_view =
            color_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mask_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("rich_text create mask_texture"),
            size: wgpu::Extent3d {
                width: context.size.width as u32,
                height: context.size.height as u32,
                depth_or_array_layers: 1,
            },
            view_formats: &[],
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
            mip_level_count: 1,
            sample_count: 1,
        });
        let mask_texture_view =
            mask_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            // mag_filter: wgpu::FilterMode::Nearest,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: 0f32,
            lod_max_clamp: 0f32,
            ..Default::default()
        });

        // Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &transform,
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&color_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&mask_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("rich_text::Pipeline uniforms"),
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
                "rich_text.wgsl"
            ))),
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            cache: None,
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    // https://docs.rs/wgpu/latest/wgpu/enum.VertexStepMode.html
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &wgpu::vertex_attr_array!(
                        0 => Float32x4,
                        1 => Float32x4,
                        2 => Float32x2,
                    ),
                }],
            },
            fragment: Some(wgpu::FragmentState {
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: context.format,
                    blend: BLEND,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("rich_text::Instances Buffer"),
            size: mem::size_of::<Vertex>() as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        RichTextBrush {
            bind_group_layout,
            color_texture_view,
            mask_texture_view,
            sampler,
            textures: FxHashMap::default(),
            comp: Compositor::new(),
            images: ImageCache::new(2048),
            glyphs: GlyphCache::new(),
            draw_layout_cache: DrawLayoutCache::default(),
            dlist,
            bind_group,
            transform,
            pipeline,
            vertex_buffer,
            first_run: true,
            bind_group_needs_update: true,
            vertices_quantity,
            current_transform,
        }
    }

    #[inline]
    pub fn clean_cache(&mut self) {
        self.draw_layout_cache.clear();
    }

    #[inline]
    pub fn prepare(
        &mut self,
        ctx: &mut Context,
        state: &crate::sugarloaf::state::SugarState,
    ) {
        // let start = std::time::Instant::now();

        if state.compositors.advanced.render_data.is_empty() {
            self.dlist.clear();
            return;
        }

        // Render
        self.comp.begin();

        let library = state.compositors.advanced.font_library();
        let font_library = { &library.inner.read().unwrap() };

        draw_layout(
            &mut self.comp,
            (
                &mut self.images,
                &mut self.glyphs,
                &mut self.draw_layout_cache,
            ),
            &state.compositors.advanced.render_data,
            state.current.layout.style.screen_position,
            font_library,
            state.current.layout.dimensions,
        );
        self.draw_layout_cache.clear_on_demand();
        // let duration = start.elapsed();
        // println!(" - rich_text::prepare::draw_layout() is: {:?}", duration);

        self.dlist.clear();
        self.finish_composition(ctx);

        // let duration = start.elapsed();
        // println!(" - rich_text::prepare() is: {:?}", duration);
    }

    #[inline]
    pub fn dimensions(
        &mut self,
        state: &crate::sugarloaf::state::SugarState,
    ) -> Option<SugarDimensions> {
        self.comp.begin();

        let library = state.compositors.advanced.font_library();
        let font_library = { &library.inner.read().unwrap() };

        let dimension = fetch_dimensions(
            &mut self.comp,
            (&mut self.images, &mut self.glyphs),
            &state.compositors.advanced.mocked_render_data,
            font_library,
        );
        if dimension.height > 0. && dimension.width > 0. {
            Some(dimension)
        } else {
            None
        }
    }

    #[inline]
    pub fn render<'pass>(
        &'pass mut self,
        ctx: &mut Context,
        state: &crate::sugarloaf::state::SugarState,
        rpass: &mut wgpu::RenderPass<'pass>,
    ) {
        // let start = std::time::Instant::now();
        let vertices: &[Vertex] = self.dlist.vertices();

        // There's nothing to render
        if vertices.is_empty() {
            return;
        }

        let queue = &mut ctx.queue;

        let transform = orthographic_projection(
            state.current.layout.width,
            state.current.layout.height,
        );
        let transform_has_changed = transform != self.current_transform;

        if transform_has_changed {
            queue.write_buffer(&self.transform, 0, bytemuck::bytes_of(&transform));
            self.current_transform = transform;
        }

        if vertices.len() > self.vertices_quantity {
            self.vertex_buffer.destroy();

            self.vertices_quantity = vertices.len();
            self.vertex_buffer = ctx.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("sugarloaf::rich_text::Pipeline instances"),
                size: mem::size_of::<Vertex>() as u64
                    * self.vertices_quantity as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }

        let vertices_bytes: &[u8] = bytemuck::cast_slice(vertices);
        if !vertices_bytes.is_empty() {
            queue.write_buffer(&self.vertex_buffer, 0, vertices_bytes);
        }

        let mut color_texture_updated: Option<&TextureId> = None;
        let mut mask_texture_updated: Option<&TextureId> = None;

        for command in self.dlist.commands() {
            match command {
                Command::BindTexture(unit, id) => {
                    if self.bind_group_needs_update {
                        match unit {
                            // color_texture
                            0 => {
                                // if color_texture_updated.is_none() {
                                if let Some(texture) = self.textures.get(id) {
                                    log::info!("rich_text::BindTexture, set color_texture_view {:?} {:?}", unit, id);
                                    self.color_texture_view = texture.create_view(
                                        &wgpu::TextureViewDescriptor::default(),
                                    );
                                    color_texture_updated = Some(id);
                                }
                                // }
                            }
                            // mask_texture
                            1 => {
                                // if mask_texture_updated.is_none() {
                                if let Some(texture) = self.textures.get(id) {
                                    log::info!("rich_text::BindTexture, set mask_texture_view {:?} {:?}", unit, id);
                                    self.mask_texture_view = texture.create_view(
                                        &wgpu::TextureViewDescriptor::default(),
                                    );
                                    mask_texture_updated = Some(id);
                                }
                                // }
                            }
                            _ => {
                                // Noop
                            }
                        }
                    };
                }
            }
        }

        // Ensure texture views are not empty in the first run
        if self.first_run && mask_texture_updated.is_none() {
            if let Some(texture) = self
                .textures
                .get(color_texture_updated.unwrap_or(&TextureId(0)))
            {
                self.mask_texture_view =
                    texture.create_view(&wgpu::TextureViewDescriptor::default());
            }
        }
        if self.first_run && color_texture_updated.is_none() {
            if let Some(texture) = self
                .textures
                .get(mask_texture_updated.unwrap_or(&TextureId(0)))
            {
                self.color_texture_view =
                    texture.create_view(&wgpu::TextureViewDescriptor::default());
            }
        }

        if self.bind_group_needs_update {
            self.bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &self.bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                            buffer: &self.transform,
                            offset: 0,
                            size: None,
                        }),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(
                            &self.color_texture_view,
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(
                            &self.mask_texture_view,
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: wgpu::BindingResource::Sampler(&self.sampler),
                    },
                ],
                label: Some("rich_text::Pipeline uniforms"),
            });
        }

        rpass.set_pipeline(&self.pipeline);
        rpass.set_bind_group(0, &self.bind_group, &[]);
        rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        rpass.draw(0..4, 0..vertices.len() as u32);

        self.bind_group_needs_update = false;
        self.first_run = false;

        // let duration = start.elapsed();
        // println!(" - rich_text::render() is: {:?}", duration);
    }

    #[inline]
    fn finish_composition(&mut self, ctx: &mut Context) {
        self.comp
            .finish(&mut self.dlist, &mut self.images, |event| {
                match event {
                    TextureEvent::CreateTexture {
                        id,
                        format,
                        width,
                        height,
                        data,
                    } => {
                        log::info!(
                            "rich_text::CreateTexture with id ({:?}) and format {:?}",
                            id,
                            format
                        );
                        let texture_size = wgpu::Extent3d {
                            width: width.into(),
                            height: height.into(),
                            depth_or_array_layers: 1,
                        };
                        let texture =
                            ctx.device.create_texture(&wgpu::TextureDescriptor {
                                size: texture_size,
                                mip_level_count: 1,
                                sample_count: 1,
                                dimension: wgpu::TextureDimension::D2,
                                format: match format {
                                    image_cache::PixelFormat::A8 => {
                                        wgpu::TextureFormat::R8Unorm
                                    }
                                    image_cache::PixelFormat::Rgba8 => {
                                        wgpu::TextureFormat::Rgba8Unorm
                                    }
                                },
                                usage: wgpu::TextureUsages::TEXTURE_BINDING
                                    | wgpu::TextureUsages::COPY_DST,
                                label: Some("rich_text::Cache"),
                                view_formats: &[],
                            });

                        if let Some(data) = data {
                            self.bind_group_needs_update = true;
                            let channels = match format {
                                // Mask
                                image_cache::PixelFormat::A8 => 1,
                                // Color
                                image_cache::PixelFormat::Rgba8 => 4,
                            };

                            ctx.queue.write_texture(
                                // Tells wgpu where to copy the pixel data
                                wgpu::ImageCopyTexture {
                                    texture: &texture,
                                    mip_level: 0,
                                    origin: wgpu::Origin3d::ZERO,
                                    aspect: wgpu::TextureAspect::All,
                                },
                                // The actual pixel data
                                data,
                                // The layout of the texture
                                wgpu::ImageDataLayout {
                                    offset: 0,
                                    bytes_per_row: Some((width * channels).into()),
                                    rows_per_image: Some(height.into()),
                                },
                                texture_size,
                            );
                        }

                        self.textures.insert(id, texture);
                    }
                    TextureEvent::UpdateTexture {
                        id,
                        format,
                        x,
                        y,
                        width,
                        height,
                        data,
                    } => {
                        log::info!("rich_text::UpdateTexture id ({:?})", id);
                        if let Some(texture) = self.textures.get(&id) {
                            self.bind_group_needs_update = true;
                            let texture_size = wgpu::Extent3d {
                                width: width.into(),
                                height: height.into(),
                                depth_or_array_layers: 1,
                            };

                            let channels = match format {
                                // Mask
                                image_cache::PixelFormat::A8 => 1,
                                // Color
                                image_cache::PixelFormat::Rgba8 => 4,
                            };

                            ctx.queue.write_texture(
                                // Tells wgpu where to copy the pixel data
                                wgpu::ImageCopyTexture {
                                    texture,
                                    mip_level: 0,
                                    origin: wgpu::Origin3d {
                                        x: u32::from(x),
                                        y: u32::from(y),
                                        z: 0,
                                    },
                                    aspect: wgpu::TextureAspect::All,
                                },
                                // The actual pixel data
                                data,
                                // The layout of the texture
                                wgpu::ImageDataLayout {
                                    offset: 0,
                                    bytes_per_row: Some((width * channels).into()),
                                    rows_per_image: Some(height.into()),
                                },
                                texture_size,
                            );
                        }
                    }
                    TextureEvent::DestroyTexture(id) => {
                        log::info!("rich_text::DestroyTexture id ({:?})", id);
                        self.textures.remove(&id);
                    }
                }
            });
    }
}

#[derive(Default)]
struct DrawLayoutCache {
    inner: FxHashMap<u64, Vec<CachedRun>>,
}

impl DrawLayoutCache {
    #[inline]
    fn get(&self, hash: &u64) -> Option<&Vec<CachedRun>> {
        self.inner.get(hash)
    }

    #[inline]
    fn insert(&mut self, hash: u64, data: Vec<CachedRun>) {
        self.inner.insert(hash, data);
    }

    #[inline]
    fn clear_on_demand(&mut self) {
        if self.inner.len() > 1024 {
            self.inner.clear();
        }
    }

    #[inline]
    fn clear(&mut self) {
        self.inner.clear();
    }
}

#[inline]
fn draw_layout(
    comp: &mut compositor::Compositor,
    caches: (&mut ImageCache, &mut GlyphCache, &mut DrawLayoutCache),
    render_data: &crate::layout::RenderData,
    pos: (f32, f32),
    font_library: &FontLibraryData,
    rect: SugarDimensions,
) {
    // let start = std::time::Instant::now();
    let (x, y) = pos;
    let (image_cache, glyphs_cache, draw_layout_cache) = caches;
    let depth = 0.0;
    let mut glyphs = Vec::new();
    let mut current_font = 0;
    let mut current_font_size = 0.0;
    let mut current_font_coords: Vec<i16> = Vec::with_capacity(0);
    if let Some(line) = render_data.lines().next() {
        if let Some(run) = line.runs().next() {
            current_font = *run.font();
            current_font_size = run.font_size();
            current_font_coords = run.normalized_coords().to_vec();
        }
    }

    let mut session = glyphs_cache.session(
        image_cache,
        font_library[current_font].as_ref(),
        &current_font_coords,
        current_font_size,
    );

    for line in render_data.lines() {
        let hash = line.hash().unwrap_or(0);
        let mut px = x + line.offset();
        let py = line.baseline() + y;
        if hash > 0 {
            if let Some(data) = draw_layout_cache.get(&hash) {
                comp.draw_cached_run(data, px, py, depth, rect, line);
                continue;
            }
        }

        let mut cached_line_runs = Vec::new();
        for run in line.runs() {
            let char_width = run.char_width();
            let mut cached_run = CachedRun::new(char_width);
            let font = *run.font();
            let char_width = run.char_width();

            let run_x = px;
            glyphs.clear();
            for cluster in run.visual_clusters() {
                for glyph in cluster.glyphs() {
                    cached_run.glyphs_ids.push(glyph.id);

                    let x = px + glyph.x;
                    let y = py - glyph.y;
                    // px += glyph.advance
                    px += rect.width * char_width;
                    glyphs.push(Glyph { id: glyph.id, x, y });
                }
            }

            let line_height = line.ascent() + line.descent() + line.leading();
            let style = TextRunStyle {
                font: font_library[font].as_ref(),
                font_coords: run.normalized_coords(),
                font_size: run.font_size(),
                color: run.color(),
                cursor: run.cursor(),
                background_color: run.background_color(),
                baseline: py,
                topline: py - line.ascent(),
                line_height,
                advance: px - run_x,
                decoration: run.decoration(),
                decoration_color: run.decoration_color(),
            };

            if font != current_font
                || style.font_size != current_font_size
                || style.font_coords != current_font_coords
            {
                session = glyphs_cache.session(
                    image_cache,
                    style.font,
                    style.font_coords,
                    style.font_size,
                );

                current_font = font;
                current_font_coords = style.font_coords.to_vec();
                current_font_size = style.font_size;
            }

            if hash > 0 {
                comp.draw_run(
                    &mut session,
                    Rect::new(run_x, py, style.advance, 1.),
                    depth,
                    &style,
                    glyphs.iter(),
                    &mut cached_run,
                );

                cached_line_runs.push(cached_run);
            }
        }

        if hash > 0 && !cached_line_runs.is_empty() {
            draw_layout_cache.insert(hash, cached_line_runs);
        }
    }

    // let duration = start.elapsed();
    // println!(" - draw_layout() is: {:?}\n", duration);
}

#[inline]
fn fetch_dimensions(
    comp: &mut compositor::Compositor,
    caches: (&mut ImageCache, &mut GlyphCache),
    render_data: &crate::layout::RenderData,
    font_library: &FontLibraryData,
) -> SugarDimensions {
    let x = 0.;
    let y = 0.;

    let (image_cache, glyphs_cache) = caches;
    let mut current_font = 0;
    let mut current_font_size = 0.0;
    let mut current_font_coords: Vec<i16> = Vec::with_capacity(0);
    if let Some(line) = render_data.lines().next() {
        if let Some(run) = line.runs().next() {
            current_font = *run.font();
            current_font_size = run.font_size();
            current_font_coords = run.normalized_coords().to_vec();
        }
    }

    let mut session = glyphs_cache.session(
        image_cache,
        font_library[current_font].as_ref(),
        &current_font_coords,
        current_font_size,
    );

    let mut glyphs = Vec::with_capacity(3);
    let mut dimension = SugarDimensions::default();
    for line in render_data.lines() {
        let mut px = x + line.offset();
        for run in line.runs() {
            let char_width = run.char_width();
            let mut cached_run = CachedRun::new(char_width);

            let font = run.font();
            let py = line.baseline() + y;
            let run_x = px;
            let line_height = line.ascent() + line.descent() + line.leading();
            glyphs.clear();
            for cluster in run.visual_clusters() {
                for glyph in cluster.glyphs() {
                    let x = px + glyph.x;
                    let y = py - glyph.y;
                    px += glyph.advance * char_width;
                    glyphs.push(Glyph { id: glyph.id, x, y });
                }
            }
            let color = run.color();

            let style = TextRunStyle {
                font: font_library[*font].as_ref(),
                font_coords: run.normalized_coords(),
                font_size: run.font_size(),
                color,
                cursor: run.cursor(),
                background_color: None,
                baseline: py,
                topline: py - line.ascent(),
                line_height,
                advance: px - run_x,
                decoration: None,
                decoration_color: None,
            };

            if style.advance > 0. && line_height > 0. {
                dimension.width = style.advance;
                dimension.height = line_height;
            }

            if font != &current_font
                || style.font_size != current_font_size
                || style.font_coords != current_font_coords
            {
                session = glyphs_cache.session(
                    image_cache,
                    style.font,
                    style.font_coords,
                    style.font_size,
                );

                current_font = *font;
                current_font_coords = style.font_coords.to_vec();
                current_font_size = style.font_size;
            }

            comp.draw_run(
                &mut session,
                Rect::new(run_x, py, style.advance, 1.),
                0.0,
                &style,
                glyphs.iter(),
                &mut cached_run,
            );
        }
    }

    dimension
}
