
#[derive(Debug)]
pub enum ShaderError {
    ShaderCompileError { message: String },
    ProgramCompileError { message: String }
}