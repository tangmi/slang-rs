#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let shaders = ShaderPipeline::from_slang(
            "shader.slang",
            r"
struct VOut
{
    float4 position : SV_POSITION;
    float4 color : COLOR;
};

VOut vs_main(float4 position : POSITION, float4 color : COLOR)
{
    VOut output;

    output.position = position;
    output.color = color;

    return output;
}


float4 ps_main(float4 position : SV_POSITION, float4 color : COLOR) : SV_TARGET
{
    return color;
}
",
            "vs_main",
            "ps_main",
        )
        .unwrap();

        println!("{}", std::str::from_utf8(&shaders.vertex).unwrap());
        println!("{}", std::str::from_utf8(&shaders.pixel).unwrap());
    }
}

#[derive(Copy, Clone, Eq, Debug, PartialEq, Hash)]
pub enum Backend {
    /// OpenGL 3.2+
    Glsl150,

    /// OpenGL ES 3.0+
    Glsl300es,

    /// DirectX 10+
    HlslSm40,
}

#[derive(Clone, Eq, Debug, PartialEq)]
pub struct ShaderPipeline {
    backend: Backend,
    vertex: Vec<u8>,
    pixel: Vec<u8>,
}

#[derive(Debug)]
pub enum Error {
    SlangError(slang::Error),
    CompilerOutput(String),
}

impl std::convert::From<slang::Error> for Error {
    fn from(t: slang::Error) -> Self {
        Error::SlangError(t)
    }
}

impl ShaderPipeline {
    pub fn from_slang(
        file_path: &str,
        shader_source: &str,
        vertex_entry_point_name: &str,
        pixel_entry_point_name: &str,
    ) -> Result<ShaderPipeline, Error> {
        struct BackendSelector {
            profile: &'static str,
            target: slang::CompileTarget,
            backend: Backend,
        }

        #[cfg(windows)]
        let selector = BackendSelector {
            profile: "sm_4_0",
            target: slang::CompileTarget::Hlsl,
            backend: Backend::HlslSm40,
        };

        #[cfg(not(windows))]
        let selector = BackendSelector {
            profile: "glsl_150",                // TODO this isn't working?
            target: slang::CompileTarget::Glsl, // spirv?
            backend: Backend::Glsl150,
        };

        let session = slang::Session::new();
        let profile_id = session
            .find_profile(std::ffi::CString::new(selector.profile).unwrap())
            .expect("profile is non-zero");

        let request = session.create_compile_request();

        let code_gen_target = request.add_code_gen_target(selector.target);

        request.set_target_profile(code_gen_target, profile_id);

        let translation_unit = request.add_translation_unit(
            slang::SourceLanguage::Slang,
            std::ffi::CString::new("shader").unwrap(),
        );

        request.add_translation_unit_source_string(
            translation_unit,
            std::ffi::CString::new(file_path).unwrap(),
            std::ffi::CString::new(shader_source).unwrap(),
        );

        let entry_point_vertex = request.add_entry_point(
            translation_unit,
            std::ffi::CString::new(vertex_entry_point_name).unwrap(),
            slang::Stage::Vertex,
        );

        let entry_point_pixel = request.add_entry_point(
            translation_unit,
            std::ffi::CString::new(pixel_entry_point_name).unwrap(),
            slang::Stage::Fragment,
        );

        let res = request.compile();

        let diagnostic_output = request.get_diagnostic_output();
        if diagnostic_output.to_bytes().len() > 1 {
            Err(Error::CompilerOutput(
                diagnostic_output.to_string_lossy().to_string(),
            ))
        } else {
            // I believe we can suceed a compile and still get diagnostic output
            res?;

            #[cfg(target_os = "emscripten")]
            {
                // TODO: convert to 300es
                unimplemented!();
            }

            #[cfg(not(windows))]
            {
                // TODO: get both entrypoints into a single spirv module?
                let module = spirv_cross::spirv::Module::from_words(&[0]);

                let mut ast =
                    spirv_cross::spirv::Ast::<spirv_cross::glsl::Target>::parse(&module).unwrap();

                ast.set_compiler_options(&spirv_cross::glsl::CompilerOptions {
                    version: spirv_cross::glsl::Version::V1_10,
                    vertex: spirv_cross::glsl::CompilerVertexOptions {
                        invert_y: false,
                        transform_clip_space: false,
                    },
                })
                .unwrap();

                dbg!(ast.get_entry_points().unwrap());

                let output = ast.compile().unwrap();

                println!("{}", output);
            }

            Ok(ShaderPipeline {
                backend: selector.backend,
                vertex: request.get_entry_point_code(entry_point_vertex).to_vec(),
                pixel: request.get_entry_point_code(entry_point_pixel).to_vec(),
            })
        }
    }
}

pub trait FactoryExt<R: gfx::Resources>: gfx::traits::FactoryExt<R> {
    fn create_shaders(
        &mut self,
        shader: ShaderPipeline,
    ) -> Result<gfx::ShaderSet<R>, gfx::shade::ProgramError> {
        #[cfg(windows)]
        assert_eq!(shader.backend, Backend::HlslSm40);

        #[cfg(target_os = "emscripten")]
        assert_eq!(shader.backend, Backend::Glsl300es);

        #[cfg(all(not(target_os = "emscripten"), not(windows)))]
        assert_eq!(shader.backend, Backend::Glsl300es);

        self.create_shader_set(&shader.vertex, &shader.pixel)
    }
}
