use GL;
use GL::Gl;
use shader::Type;
use shader::shader_error::ShaderError;
use std::os::raw::c_char;
use math::{Vec3, Mat4};

impl Shader {
    pub fn new(gl: &Gl, shader_type: Type, include_geometry: bool) -> Result<Shader, ShaderError> {
        unsafe {
            let program = gl.CreateProgram();
            let shader_str = shader_type.file_bytes_as_cstr(include_geometry).unwrap();

            let v = gl.CreateShader(GL::VERTEX_SHADER);
            let f = gl.CreateShader(GL::FRAGMENT_SHADER);

            gl.ShaderSource(v, 1, &shader_str.vertex.as_ptr(), ::std::ptr::null());
            gl.ShaderSource(f, 1, &shader_str.fragment.as_ptr(), ::std::ptr::null());

            gl.CompileShader(v);
            get_shader_status(v, gl)?;
            gl.CompileShader(f);
            get_shader_status(f, gl)?;

            gl.AttachShader(program, v);
            gl.AttachShader(program, f);
            let mut g = 0;
            if include_geometry {
                g = gl.CreateShader(GL::GEOMETRY_SHADER);
                gl.ShaderSource(g, 1, &shader_str.geometry.unwrap().as_ptr(), ::std::ptr::null());
                gl.CompileShader(g);
                get_shader_status(g, gl)?;
                gl.AttachShader(program, g);
            }
            gl.LinkProgram(program);
            get_program_status(program, gl)?;
            gl.DeleteShader(v);
            gl.DeleteShader(f);
            if include_geometry {
                gl.DeleteShader(g);
            }

            Ok(Shader { program, gl: gl.clone() })
        }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.UseProgram(self.program);
        }
    }

    pub fn mat_4(&self, name: &str, value: &Mat4) {
        unsafe {
            self.gl.UniformMatrix4fv(self.get_location(name), 1, GL::FALSE, &value[0][0] as *const f32);
        }
    }

    pub fn vec3(&self, name: &str, value: &Vec3) {
        unsafe {
            self.gl.Uniform3fv(self.get_location(name), 1, &value[0]);
        }
    }

    pub fn int(&self, name: &str, value: i32) {
        unsafe {
            self.gl.Uniform1i(self.get_location(name), value);
        }
    }

    pub fn float(&self, name: &str, value: f32) {
        unsafe {
            self.gl.Uniform1f(self.get_location(name), value);
        }
    }

    pub fn get_location(&self, name: &str) -> i32 {
        unsafe {
            let cname = ::std::ffi::CString::new(name)
                .expect(format!("Expected uniform name, got: {}", name).as_str());

            self.gl.GetUniformLocation(self.program, cname.as_bytes_with_nul().as_ptr() as *const i8)
        }
    }
}

fn get_program_status(program: u32, gl: &Gl) -> Result<(), ShaderError> {
    unsafe {
        let mut success = GL::FALSE as i32;
        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512 - 1);
        gl.GetProgramiv(program, GL::LINK_STATUS, &mut success);
        if success != GL::TRUE as i32 {
            gl.GetProgramInfoLog(program, 512, ::std::ptr::null_mut(), info_log.as_mut_ptr() as *mut c_char);
            return Err(ShaderError::ShaderCompileError {
                message: format!("Program link error: {}", ::std::str::from_utf8(&info_log).unwrap())
            });
        }
    }
    Ok(())
}

fn get_shader_status(shader: u32, gl: &Gl) -> Result<(), ShaderError> {
    unsafe {
        let mut success = GL::FALSE as i32;
        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512 - 1);
        gl.GetShaderiv(shader, GL::COMPILE_STATUS, &mut success);
        if success != GL::TRUE as i32 {
            gl.GetShaderInfoLog(shader, 512, ::std::ptr::null_mut(), info_log.as_mut_ptr() as *mut c_char);
            return Err(ShaderError::ProgramCompileError {
                message: format!("Shader compile error: {}", ::std::str::from_utf8(&info_log).unwrap())
            });
        }
        Ok(())
    }
}

pub struct Shader {
    program: u32,
    gl: Gl
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.program);
        }
    }
}