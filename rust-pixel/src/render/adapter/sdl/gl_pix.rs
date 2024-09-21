use crate::render::adapter::sdl::gl_color::GlColor;
use crate::render::adapter::sdl::gl_shader::GlShader;
use crate::render::adapter::sdl::gl_shader::GlShaderCore;
use crate::render::adapter::sdl::gl_shader::GlUniformValue;
use crate::render::adapter::sdl::gl_texture::GlFrame;
use crate::render::adapter::sdl::gl_texture::GlTexture;
use crate::render::adapter::sdl::gl_transform::GlTransform;
use glow::HasContext;
use std::collections::HashMap;
// use log::info;

#[derive(Clone, Copy, PartialEq)]
pub enum GlRenderMode {
    None = -1,
    PixCells = 0,
}

pub struct GlPix {
    // 着色器列表
    pub shader_core_cells: GlShaderCore,
    pub shaders: Vec<GlShader>,

    // 变换栈
    pub transform_stack: Vec<GlTransform>,
    pub transform_at: usize,
    pub transform_dirty: bool,

    // 实例缓冲区
    pub instance_buffer: Vec<f32>,
    pub instance_buffer_capacity: usize,
    pub instance_buffer_at: isize,
    pub instance_count: usize,

    // 渲染模式
    pub render_mode: GlRenderMode,

    // OpenGL 缓冲区和顶点数组对象
    pub vao_cells: glow::NativeVertexArray,
    pub instances_vbo: glow::NativeBuffer,
    pub quad_vbo: glow::NativeBuffer,
    pub ubo: glow::NativeBuffer,

    // Uniform Buffer 内容
    pub ubo_contents: [f32; 12],

    // 当前状态
    pub current_shader: Option<usize>,
    pub current_shader_core: Option<usize>,
    pub current_texture_atlas: Option<glow::NativeTexture>,

    // 画布尺寸
    pub canvas_width: u32,
    pub canvas_height: u32,

    // 清除颜色
    pub clear_color: GlColor,
}

impl GlPix {
    pub fn new(gl: &glow::Context, canvas_width: i32, canvas_height: i32) -> Self {
        // 初始化着色器
        let vertex_shader_src = r#"
        #version 330 core
        layout(location=0) in vec2 vertex;
        layout(location=1) in vec4 a1;
        layout(location=2) in vec4 a2;
        layout(location=3) in vec4 a3;
        layout(location=4) in vec4 color;
        layout(std140) uniform transform {
            vec4 tw;
            vec4 th;
            vec4 colorFilter;
        };
        out vec2 uv;
        out vec4 colorj;
        void main() {
            uv = a1.zw + vertex * a2.xy;
            vec2 transformed = (((vertex - a1.xy) * mat2(a2.zw, a3.xy) + a3.zw) * mat2(tw.xy, th.xy) + vec2(tw.z, th.z)) / vec2(tw.w, th.w) * 2.0;
            gl_Position = vec4(transformed - vec2(1.0, 1.0), 0.0, 1.0);
            colorj = color * colorFilter;
        }
        "#;

        let fragment_shader_src = r#"
        #version 330 core
        uniform sampler2D source;
        layout(std140) uniform transform {
            vec4 tw;
            vec4 th;
            vec4 colorFilter;
        };
        in vec2 uv;
        in vec4 colorj;
        layout(location=0) out vec4 color;
        void main() {
            color = texture(source, uv) * colorj;
        }
        "#;

        let shader_core_cells = GlShaderCore::new(&gl, vertex_shader_src, fragment_shader_src);

        let mut uniforms = HashMap::new();
        uniforms.insert("source".to_string(), GlUniformValue::Int(0));

        let shader = GlShader::new(shader_core_cells.clone(), uniforms);

        let shaders = vec![shader];

        // 创建缓冲区和 VAO
        let quad_vbo = unsafe { gl.create_buffer().unwrap() };
        let instances_vbo = unsafe { gl.create_buffer().unwrap() };
        let vao_cells = unsafe { gl.create_vertex_array().unwrap() };
        let ubo = unsafe { gl.create_buffer().unwrap() };

        // 初始化缓冲区
        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(instances_vbo));
            let instance_buffer_capacity = 1024;
            gl.buffer_data_size(
                glow::ARRAY_BUFFER,
                (instance_buffer_capacity * std::mem::size_of::<f32>()) as i32,
                glow::DYNAMIC_DRAW,
            );

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(quad_vbo));
            let quad_vertices: [f32; 8] = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0];
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                &quad_vertices.align_to::<u8>().1,
                glow::STATIC_DRAW,
            );

            gl.bind_buffer(glow::UNIFORM_BUFFER, Some(ubo));
            gl.buffer_data_size(glow::UNIFORM_BUFFER, 48, glow::DYNAMIC_DRAW);
            gl.bind_buffer_base(glow::UNIFORM_BUFFER, 0, Some(ubo));

            // 设置 VAO
            gl.bind_vertex_array(Some(vao_cells));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(quad_vbo));
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 8, 0);

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(instances_vbo));

            let stride = 64;

            // Attribute 1
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(1, 4, glow::FLOAT, false, stride, 0);
            gl.vertex_attrib_divisor(1, 1);

            // Attribute 2
            gl.enable_vertex_attrib_array(2);
            gl.vertex_attrib_pointer_f32(2, 4, glow::FLOAT, false, stride, 16);
            gl.vertex_attrib_divisor(2, 1);

            // Attribute 3
            gl.enable_vertex_attrib_array(3);
            gl.vertex_attrib_pointer_f32(3, 4, glow::FLOAT, false, stride, 32);
            gl.vertex_attrib_divisor(3, 1);

            // Attribute 4 (color)
            gl.enable_vertex_attrib_array(4);
            gl.vertex_attrib_pointer_f32(4, 4, glow::FLOAT, false, stride, 48);
            gl.vertex_attrib_divisor(4, 1);

            gl.bind_vertex_array(None);

            // 启用混合
            gl.enable(glow::BLEND);
            gl.disable(glow::DEPTH_TEST);
            gl.blend_func_separate(
                glow::SRC_ALPHA,
                glow::ONE_MINUS_SRC_ALPHA,
                glow::ONE,
                glow::ONE_MINUS_SRC_ALPHA,
            );
        }

        let mut ubo_contents = [0.0f32; 12];
        ubo_contents[8] = 1.0;
        ubo_contents[9] = 1.0;
        ubo_contents[10] = 1.0;
        ubo_contents[11] = 1.0;

        Self {
            canvas_width: canvas_width as u32,
            canvas_height: canvas_height as u32,
            shader_core_cells,
            shaders,
            quad_vbo,
            instances_vbo,
            vao_cells,
            ubo,
            ubo_contents,
            transform_stack: vec![GlTransform::new_with_values(
                1.0,
                0.0,
                0.0,
                0.0,
                -1.0,
                canvas_height as f32,
            )],
            transform_at: 0,
            transform_dirty: true,
            instance_buffer_capacity: 1024,
            instance_buffer_at: -1,
            instance_buffer: vec![0.0; 1024],
            instance_count: 0,
            render_mode: GlRenderMode::None,
            current_shader: None,
            current_shader_core: None,
            current_texture_atlas: None,
            clear_color: GlColor::new(1.0, 1.0, 1.0, 0.0),
        }
    }

    pub fn prepare_draw(&mut self, gl: &glow::Context, mode: GlRenderMode, size: usize) {
        if self.transform_dirty {
            self.flush(gl);
            self.send_uniform_buffer(gl);
        }

        if self.render_mode != mode {
            self.flush(gl);
            self.render_mode = mode;
            self.shaders[mode as usize].bind(gl);
        }

        if (self.instance_buffer_at + size as isize) as usize >= self.instance_buffer_capacity {
            self.instance_buffer_capacity *= 2;
            self.instance_buffer
                .resize(self.instance_buffer_capacity, 0.0);

            unsafe {
                gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.instances_vbo));
                gl.buffer_data_size(
                    glow::ARRAY_BUFFER,
                    (self.instance_buffer_capacity * std::mem::size_of::<f32>()) as i32,
                    glow::DYNAMIC_DRAW,
                );
            }
        }

        self.instance_count += 1;
    }

    fn send_uniform_buffer(&mut self, gl: &glow::Context) {
        let transform = self.transform_stack.last().unwrap();
        self.ubo_contents[0] = transform.m00;
        self.ubo_contents[1] = transform.m10;
        self.ubo_contents[2] = transform.m20;
        self.ubo_contents[4] = transform.m01;
        self.ubo_contents[5] = transform.m11;
        self.ubo_contents[6] = transform.m21;
        self.ubo_contents[3] = self.canvas_width as f32;
        self.ubo_contents[7] = self.canvas_height as f32;

        unsafe {
            gl.bind_buffer(glow::UNIFORM_BUFFER, Some(self.ubo));
            gl.buffer_sub_data_u8_slice(
                glow::UNIFORM_BUFFER,
                0,
                &self.ubo_contents.align_to::<u8>().1,
            );
        }

        self.transform_dirty = false;
    }

    pub fn bind(&mut self, gl: &glow::Context) {
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
            gl.viewport(0, 0, self.canvas_width as i32, self.canvas_height as i32);
        }
    }

    pub fn clear(&mut self, gl: &glow::Context) {
        self.flush(gl);

        unsafe {
            gl.clear_color(
                self.clear_color.r * self.ubo_contents[8],
                self.clear_color.g * self.ubo_contents[9],
                self.clear_color.b * self.ubo_contents[10],
                self.clear_color.a * self.ubo_contents[11],
            );
            gl.clear(glow::COLOR_BUFFER_BIT);
        }
    }

    pub fn flush(&mut self, gl: &glow::Context) {
        if self.instance_count == 0 {
            return;
        }

        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.instances_vbo));
            gl.buffer_sub_data_u8_slice(
                glow::ARRAY_BUFFER,
                0,
                &self.instance_buffer[0..=(self.instance_buffer_at as usize)]
                    .align_to::<u8>()
                    .1,
            );

            gl.bind_vertex_array(Some(self.vao_cells));
            gl.draw_arrays_instanced(glow::TRIANGLE_FAN, 0, 4, self.instance_count as i32);

            self.instance_buffer_at = -1;
            self.instance_count = 0;
        }
    }

    pub fn bind_texture_atlas(&mut self, gl: &glow::Context, texture: glow::NativeTexture) {
        if Some(texture) == self.current_texture_atlas {
            return;
        }

        self.flush(gl);

        unsafe {
            gl.active_texture(glow::TEXTURE0);
            gl.bind_texture(glow::TEXTURE_2D, Some(texture));
        }

        self.current_texture_atlas = Some(texture);
    }

    pub fn make_cell_frame(
        &mut self,
        sheet: &mut GlTexture,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        x_origin: f32,
        y_origin: f32,
    ) -> GlFrame {
        let origin_x = x_origin / width;
        let origin_y = y_origin / height;
        let tex_width = sheet.width as f32;
        let tex_height = sheet.height as f32;

        let uv_left = x / tex_width;
        let uv_top = y / tex_height;
        let uv_right = width / tex_width;
        let uv_bottom = height / tex_height;

        let frame = GlFrame {
            texture: sheet.texture,
            width,
            height,
            origin_x,
            origin_y,
            uv_left,
            uv_top,
            uv_right,
            uv_bottom,
        };

        frame
    }

    pub fn set_clear_color(&mut self, color: GlColor) {
        self.clear_color = color;
    }
}
