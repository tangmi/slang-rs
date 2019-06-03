use bitflags::bitflags;

// NOTE: These need to be kept up-to-date with the enums in `slang.h`.
// TODO: Is there a way to automatically generate these with `bindgen`?

#[repr(i32)]
pub enum Severity {
    Note = slang_sys::SLANG_SEVERITY_NOTE,
    Warning = slang_sys::SLANG_SEVERITY_WARNING,
    Error = slang_sys::SLANG_SEVERITY_ERROR,
    Fatal = slang_sys::SLANG_SEVERITY_FATAL,
    Internal = slang_sys::SLANG_SEVERITY_INTERNAL,
}

#[repr(i32)]
pub enum BindableResourceType {
    NonBindable = slang_sys::SLANG_NON_BINDABLE,
    Texture = slang_sys::SLANG_TEXTURE,
    Sampler = slang_sys::SLANG_SAMPLER,
    UniformBuffer = slang_sys::SLANG_UNIFORM_BUFFER,
    StorageBuffer = slang_sys::SLANG_STORAGE_BUFFER,
}

#[repr(i32)]
pub enum CompileTarget {
    TargetUnknown = slang_sys::SLANG_TARGET_UNKNOWN,
    TargetNone = slang_sys::SLANG_TARGET_NONE,
    Glsl = slang_sys::SLANG_GLSL,
    GlslVulkan = slang_sys::SLANG_GLSL_VULKAN,
    GlslVulkanOneDesc = slang_sys::SLANG_GLSL_VULKAN_ONE_DESC,
    Hlsl = slang_sys::SLANG_HLSL,
    Spirv = slang_sys::SLANG_SPIRV,
    SpirvAsm = slang_sys::SLANG_SPIRV_ASM,
    Dxbc = slang_sys::SLANG_DXBC,
    DxbcAsm = slang_sys::SLANG_DXBC_ASM,
    Dxil = slang_sys::SLANG_DXIL,
    DxilAsm = slang_sys::SLANG_DXIL_ASM,
}

#[repr(i32)]
pub enum ContainerFormat {
    ContainerFormatNone = slang_sys::SLANG_CONTAINER_FORMAT_NONE,
    ContainerFormatSlangModule = slang_sys::SLANG_CONTAINER_FORMAT_SLANG_MODULE,
}

#[repr(i32)]
pub enum PassThrough {
    None = slang_sys::SLANG_PASS_THROUGH_NONE,
    Fxc = slang_sys::SLANG_PASS_THROUGH_FXC,
    Dxc = slang_sys::SLANG_PASS_THROUGH_DXC,
    Glslang = slang_sys::SLANG_PASS_THROUGH_GLSLANG,
}

bitflags! {
    pub struct CompileFlags: i32 {
        const NO_MANGLING = slang_sys::SLANG_COMPILE_FLAG_NO_MANGLING;
        const NO_CODEGEN = slang_sys::SLANG_COMPILE_FLAG_NO_CODEGEN;
        const NO_CHECKING = slang_sys::SLANG_COMPILE_FLAG_NO_CHECKING;
        const SPLIT_MIXED_TYPES = slang_sys::SLANG_COMPILE_FLAG_SPLIT_MIXED_TYPES;
    }
}

bitflags! {
    pub struct TargetFlags: i32 {
        const PARAMETER_BLOCKS_USE_REGISTER_SPACES = slang_sys::SLANG_TARGET_FLAG_PARAMETER_BLOCKS_USE_REGISTER_SPACES;
    }
}

#[repr(i32)]
pub enum FloatingPointMode {
    Default = slang_sys::SLANG_FLOATING_POINT_MODE_DEFAULT,
    Fast = slang_sys::SLANG_FLOATING_POINT_MODE_FAST,
    Precise = slang_sys::SLANG_FLOATING_POINT_MODE_PRECISE,
}

#[repr(i32)]
pub enum LineDirectiveMode {
    Default = slang_sys::SLANG_LINE_DIRECTIVE_MODE_DEFAULT,
    None = slang_sys::SLANG_LINE_DIRECTIVE_MODE_NONE,
    Standard = slang_sys::SLANG_LINE_DIRECTIVE_MODE_STANDARD,
    Glsl = slang_sys::SLANG_LINE_DIRECTIVE_MODE_GLSL,
}

#[repr(i32)]
pub enum SourceLanguage {
    Unknown = slang_sys::SLANG_SOURCE_LANGUAGE_UNKNOWN,
    Slang = slang_sys::SLANG_SOURCE_LANGUAGE_SLANG,
    Hlsl = slang_sys::SLANG_SOURCE_LANGUAGE_HLSL,
    Glsl = slang_sys::SLANG_SOURCE_LANGUAGE_GLSL,
}

#[repr(i32)]
pub enum MatrixLayoutMode {
    Unknown = slang_sys::SLANG_MATRIX_LAYOUT_MODE_UNKNOWN,
    RowMajor = slang_sys::SLANG_MATRIX_LAYOUT_ROW_MAJOR,
    ColumnMajor = slang_sys::SLANG_MATRIX_LAYOUT_COLUMN_MAJOR,
}

#[repr(i32)]
pub enum Stage {
    None = slang_sys::SLANG_STAGE_NONE,
    Vertex = slang_sys::SLANG_STAGE_VERTEX,
    Hull = slang_sys::SLANG_STAGE_HULL,
    Domain = slang_sys::SLANG_STAGE_DOMAIN,
    Geometry = slang_sys::SLANG_STAGE_GEOMETRY,
    Fragment = slang_sys::SLANG_STAGE_FRAGMENT,
    Compute = slang_sys::SLANG_STAGE_COMPUTE,
    RayGeneration = slang_sys::SLANG_STAGE_RAY_GENERATION,
    Intersection = slang_sys::SLANG_STAGE_INTERSECTION,
    AnyHit = slang_sys::SLANG_STAGE_ANY_HIT,
    ClosestHit = slang_sys::SLANG_STAGE_CLOSEST_HIT,
    Miss = slang_sys::SLANG_STAGE_MISS,
    Callable = slang_sys::SLANG_STAGE_CALLABLE,
    // Pixel = slang_sys::SLANG_STAGE_PIXEL, // alias for `Fragment`
}

#[repr(i32)]
pub enum DebugInfoLevel {
    None = slang_sys::SLANG_DEBUG_INFO_LEVEL_NONE,
    Minimal = slang_sys::SLANG_DEBUG_INFO_LEVEL_MINIMAL,
    Standard = slang_sys::SLANG_DEBUG_INFO_LEVEL_STANDARD,
    Maximal = slang_sys::SLANG_DEBUG_INFO_LEVEL_MAXIMAL,
}

#[repr(i32)]
pub enum OptimizationLevel {
    None = slang_sys::SLANG_OPTIMIZATION_LEVEL_NONE,
    Default = slang_sys::SLANG_OPTIMIZATION_LEVEL_DEFAULT,
    High = slang_sys::SLANG_OPTIMIZATION_LEVEL_HIGH,
    Maximal = slang_sys::SLANG_OPTIMIZATION_LEVEL_MAXIMAL,
}
