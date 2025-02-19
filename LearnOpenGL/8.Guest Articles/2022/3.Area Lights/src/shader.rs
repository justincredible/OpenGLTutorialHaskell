pub mod shader {
    use glam::{Mat3, Mat4, Vec2, Vec3, Vec4};
    use glfw::with_c_str;
    use std::{fs::File, io::Read, ptr, str};

    const LOG_SIZE: usize = 1024;

    struct Shader {
        shader: gl::types::GLuint,
    }

    impl Shader {
        pub fn new(shader_type: gl::types::GLenum) -> Self {
            let shader = unsafe { gl::CreateShader(shader_type) };
            Shader { shader }
        }

        pub fn compile(self, source: &[u8]) -> Self {
            let shader = self.shader;

            let mut success: i32 = 0;

            unsafe {
                gl::ShaderSource(shader, 1, [source.as_ptr().cast()].as_ptr(), ptr::null());
                gl::CompileShader(shader);

                gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            }

            if success == 0 {
                let mut info_log = [0; LOG_SIZE];
                unsafe {
                    gl::GetShaderInfoLog(shader, LOG_SIZE as i32, ptr::null_mut(), info_log.as_mut_ptr());
                }
                println!(
                    "{}",
                    str::from_utf8(&info_log.iter().take_while(|&i| *i > 0).map(|i| *i as u8).collect::<Vec<_>>()).unwrap()
                );
            }

            self
        }
    }

    impl Drop for Shader {
        fn drop(&mut self) {
            unsafe {
                gl::DeleteShader(self.shader);
            }
        }
    }

    pub struct Program {
        program: gl::types::GLuint,
    }

    impl Program {
        pub fn new() -> Self {
            let program = unsafe { gl::CreateProgram() };
            Program { program }
        }

        pub fn link(self, vertex_file: &str, fragment_file: &str) -> Self {
            let program = self.program;

            let mut vertex_source = Vec::new();
            File::open(vertex_file).unwrap().read_to_end(&mut vertex_source).unwrap();
            vertex_source.push(0);
            let vertex_shader = Shader::new(gl::VERTEX_SHADER).compile(&vertex_source);

            let mut fragment_source = Vec::new();
            File::open(fragment_file).unwrap().read_to_end(&mut fragment_source).unwrap();
            fragment_source.push(0);
            let fragment_shader = Shader::new(gl::FRAGMENT_SHADER).compile(&fragment_source);

            let mut success: i32 = 0;

            unsafe {
                gl::AttachShader(program, vertex_shader.shader);
                gl::AttachShader(program, fragment_shader.shader);
                gl::LinkProgram(program);
                gl::DetachShader(program, vertex_shader.shader);
                gl::DetachShader(program, fragment_shader.shader);

                gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            }

            if success == 0 {
                let mut info_log = [0; LOG_SIZE];
                unsafe {
                    gl::GetProgramInfoLog(program, LOG_SIZE as i32, ptr::null_mut(), info_log.as_mut_ptr());
                }
                println!(
                    "{}",
                    str::from_utf8(&info_log.iter().take_while(|&i| *i > 0).map(|i| *i as u8).collect::<Vec<_>>()).unwrap()
                );
            }

            self
        }

        pub fn link_compute(self, compute_file: &str) -> Self {
            let program = self.program;

            let mut compute_source = Vec::new();
            File::open(compute_file).unwrap().read_to_end(&mut compute_source).unwrap();
            compute_source.push(0);
            let compute_shader = Shader::new(gl::COMPUTE_SHADER).compile(&compute_source);

            let mut success: i32 = 0;

            unsafe {
                gl::AttachShader(program, compute_shader.shader);
                gl::LinkProgram(program);
                gl::DetachShader(program, compute_shader.shader);

                gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            }

            if success == 0 {
                let mut info_log = [0; LOG_SIZE];
                unsafe {
                    gl::GetProgramInfoLog(program, LOG_SIZE as i32, ptr::null_mut(), info_log.as_mut_ptr());
                }
                println!(
                    "{}",
                    str::from_utf8(&info_log.iter().take_while(|&i| *i > 0).map(|i| *i as u8).collect::<Vec<_>>()).unwrap()
                );
            }

            self
        }

        pub fn apply(&self) {
            unsafe {
                gl::UseProgram(self.program);
            }
        }

        pub fn set_int(&self, name: &str, value: i32) {
            with_c_str(name, |locn| unsafe { gl::Uniform1i(gl::GetUniformLocation(self.program, locn), value) });
        }

        pub fn set_float(&self, name: &str, value: f32) {
            with_c_str(name, |locn| unsafe {
                gl::Uniform1f(gl::GetUniformLocation(self.program, locn), value);
            });
        }

        pub fn set_vec2(&self, name: &str, vector: Vec2) {
            let vectorray = [vector.x, vector.y];

            with_c_str(name, |locn| unsafe {
                gl::Uniform2fv(gl::GetUniformLocation(self.program, locn), 1, vectorray.as_ptr());
            });
        }

        pub fn set_vec3(&self, name: &str, vector: Vec3) {
            let vectorray = [vector.x, vector.y, vector.z];

            with_c_str(name, |locn| unsafe {
                gl::Uniform3fv(gl::GetUniformLocation(self.program, locn), 1, vectorray.as_ptr());
            });
        }

        pub fn set_vec4(&self, name: &str, vector: Vec4) {
            let vectorray = [vector.x, vector.y, vector.z, vector.w];

            with_c_str(name, |locn| unsafe {
                gl::Uniform4fv(gl::GetUniformLocation(self.program, locn), 1, vectorray.as_ptr());
            });
        }

        pub fn set_mat3(&self, name: &str, matrix: Mat3) {
            with_c_str(name, |locn| unsafe {
                gl::UniformMatrix3fv(gl::GetUniformLocation(self.program, locn), 1, gl::FALSE, matrix.to_cols_array().as_ptr());
            });
        }

        pub fn set_mat4(&self, name: &str, matrix: Mat4) {
            with_c_str(name, |locn| unsafe {
                gl::UniformMatrix4fv(gl::GetUniformLocation(self.program, locn), 1, gl::FALSE, matrix.to_cols_array().as_ptr());
            });
        }

        pub fn set_vec_vec3(&self, name: &str, vectors: &Vec<Vec3>) {
            let vectorray = vectors.iter().map(|vector| [vector.x, vector.y, vector.z]).flatten().collect::<Vec<f32>>();

            with_c_str(name, |locn| unsafe {
                gl::Uniform3fv(gl::GetUniformLocation(self.program, locn), vectors.len() as i32, vectorray.as_ptr());
            });
        }

        pub fn set_block(&self, name: &str, index: u32) {
            let id = self.program;

            with_c_str(name, |locn| unsafe {
                gl::UniformBlockBinding(id, gl::GetUniformBlockIndex(id, locn), index);
            });
        }

        pub fn program(&self) -> gl::types::GLuint {
            self.program
        }
    }

    impl Drop for Program {
        fn drop(&mut self) {
            unsafe {
                gl::DeleteProgram(self.program);
            }
        }
    }
}
