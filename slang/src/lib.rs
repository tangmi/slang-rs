#![allow(dead_code)]
#![warn(clippy::all)]
// #![warn(clippy::pedantic)]

use slang_sys::*;
use std::ffi::CStr;
use std::num::NonZeroU32;
use std::ptr;

mod enums;
#[macro_use]
mod macros;
mod result;

pub use enums::*;
pub use slang_sys as ffi;

use result::into_result;
pub use result::Error;
pub use result::Result;

lifetime_wrapper_struct!(Session, *mut SlangSession);

impl Default for Session<'_> {
    fn default() -> Self {
        unsafe { spCreateSession(ptr::null()).into() }
    }
}

impl Session<'_> {
    pub fn new() -> Self {
        Self::default()
    }

    // pub fn set_shared_library_loader(&self, loader: ISlangSharedLibraryLoader) {
    //     unimplemented!()
    // }

    // pub fn get_shared_library_loader(&self) -> ISlangSharedLibraryLoader {
    //     unimplemented!()
    // }

    // bool?
    pub fn check_compile_target_support(&self, target: CompileTarget) -> Result<()> {
        unsafe {
            into_result(spSessionCheckCompileTargetSupport(
                self.get(),
                target as SlangCompileTarget,
            ))
        }
    }

    // bool?
    pub fn check_pass_through_support(&self, pass_through: PassThrough) -> Result<()> {
        unsafe {
            into_result(spSessionCheckPassThroughSupport(
                self.get(),
                pass_through as SlangPassThrough,
            ))
        }
    }

    pub fn add_builtins(&self, source_path: impl AsRef<CStr>, source_string: impl AsRef<CStr>) {
        unsafe {
            spAddBuiltins(
                self.get(),
                source_path.as_ref().as_ptr(),
                source_string.as_ref().as_ptr(),
            )
        }
    }

    pub fn create_compile_request(&self) -> CompileRequest {
        unsafe { spCreateCompileRequest(self.get()).into() }
    }

    pub fn find_profile(&self, name: impl AsRef<CStr>) -> Option<ProfileId> {
        unsafe {
            let profile = spFindProfile(self.get(), name.as_ref().as_ptr());
            NonZeroU32::new(profile)
        }
    }
}

impl Drop for Session<'_> {
    fn drop(&mut self) {
        unsafe {
            spDestroySession(self.get());
        }
    }
}

type DiagnosticCallback = unsafe extern "C" fn(
    message: *const ::std::os::raw::c_char,
    user_data: *mut ::std::os::raw::c_void,
);

lifetime_wrapper_struct_copy!(CodeGenTarget, i32);
lifetime_wrapper_struct_copy!(TranslationUnitIndex, i32);
lifetime_wrapper_struct_copy!(EntryPointIndex, i32);

lifetime_wrapper_struct!(CompileRequest, *mut SlangCompileRequest);

// TODO there's got to be a better API shape that helps inform of usage... builder pattern maybe?
impl<'a> CompileRequest<'a> {
    // pub fn set_file_system(&self, ISlangFileSystem* fileSystem) {}

    pub fn set_compile_flags(&self, flags: CompileFlags) {
        unsafe {
            spSetCompileFlags(self.get(), flags.bits() as u32);
        }
    }

    pub fn set_dump_intermediates(&self, enable: bool) {
        unsafe {
            spSetDumpIntermediates(self.get(), if enable { 1 } else { 0 });
        }
    }

    pub fn set_line_directive_mode(&self, mode: LineDirectiveMode) {
        unsafe {
            spSetLineDirectiveMode(self.get(), mode as u32);
        }
    }

    pub fn set_code_gen_target(&self, target: CodeGenTarget) {
        unsafe {
            spSetCodeGenTarget(self.get(), target.get());
        }
    }

    pub fn add_code_gen_target(&self, target: CompileTarget) -> CodeGenTarget {
        unsafe { spAddCodeGenTarget(self.get(), target as SlangCompileTarget).into() }
    }

    pub fn set_target_profile(&self, target_index: CodeGenTarget, profile: ProfileId) {
        unsafe {
            spSetTargetProfile(self.get(), target_index.get(), profile.get());
        }
    }

    pub fn set_target_flags(&self, target_index: CodeGenTarget, flags: TargetFlags) {
        unsafe {
            spSetTargetFlags(self.get(), target_index.get(), flags.bits() as u32);
        }
    }

    pub fn set_target_floating_point_mode(
        &self,
        target_index: CodeGenTarget,
        mode: FloatingPointMode,
    ) {
        unsafe {
            spSetTargetFloatingPointMode(self.get(), target_index.get(), mode as u32);
        }
    }

    pub fn set_target_matrix_layout_mode(
        &self,
        target_index: CodeGenTarget,
        mode: MatrixLayoutMode,
    ) {
        unsafe {
            spSetTargetMatrixLayoutMode(self.get(), target_index.get(), mode as u32);
        }
    }

    pub fn set_matrix_layout_mode(&self, mode: MatrixLayoutMode) {
        unsafe {
            spSetMatrixLayoutMode(self.get(), mode as u32);
        }
    }

    pub fn set_output_container_format(&self, format: ContainerFormat) {
        unsafe {
            spSetOutputContainerFormat(self.get(), format as i32);
        }
    }

    pub fn set_pass_through(&self, pass_through: PassThrough) {
        unsafe {
            spSetPassThrough(self.get(), pass_through as i32);
        }
    }

    // TODO: is there a better way of getting a callback passed in? thread local `unsafe extern "c"` that rust assigns to before each call?
    pub unsafe fn set_diagnostic_callback(
        &self,
        callback: DiagnosticCallback,
        user_data: *mut std::ffi::c_void,
    ) {
        spSetDiagnosticCallback(self.get(), Some(callback), user_data);
    }

    // pub fn set_writer(&self,  channel:WriterChannel, ISlangWriter* writer) {unsafe { spSetWriter(self.get(),  channel:WriterChannel, ISlangWriter* writer); } }
    // pub fn get_writer(&self,  channel:WriterChannel) -> ISlangWriter*  {unsafe { spGetWriter(self.get(),  channel:WriterChannel) -> ISlangWriter* ; } }

    pub fn add_search_path(&self, search_dir: impl AsRef<CStr>) {
        unsafe {
            spAddSearchPath(self.get(), search_dir.as_ref().as_ptr());
        }
    }

    pub fn add_preprocessor_define(&self, key: impl AsRef<CStr>, value: impl AsRef<CStr>) {
        unsafe {
            spAddPreprocessorDefine(self.get(), key.as_ref().as_ptr(), value.as_ref().as_ptr());
        }
    }

    // pub fn process_command_line_arguments(&self, cstr const* args, int argCount) -> SlangResult  { unsafe { spProcessCommandLineArguments(self.get(), cstr const* args, int argCount) } }

    pub fn add_translation_unit(
        &self,
        language: SourceLanguage,
        name: impl AsRef<CStr>,
    ) -> TranslationUnitIndex {
        unsafe {
            spAddTranslationUnit(
                self.get(),
                language as SlangSourceLanguage,
                name.as_ref().as_ptr(),
            )
            .into()
        }
    }

    pub fn translation_unit_add_preprocessor_define(
        &self,
        translation_unit_index: TranslationUnitIndex,
        key: impl AsRef<CStr>,
        value: impl AsRef<CStr>,
    ) {
        unsafe {
            spTranslationUnit_addPreprocessorDefine(
                self.get(),
                translation_unit_index.get(),
                key.as_ref().as_ptr(),
                value.as_ref().as_ptr(),
            );
        }
    }

    // pub fn add_translation_unit_source_file(&self, translation_unit_index: TranslationUnitIndex, cstr path) {}

    pub fn add_translation_unit_source_string(
        &self,
        translation_unit_index: TranslationUnitIndex,
        path: impl AsRef<CStr>,
        source: impl AsRef<CStr>,
    ) {
        unsafe {
            spAddTranslationUnitSourceString(
                self.get(),
                translation_unit_index.get(),
                path.as_ref().as_ptr(),
                source.as_ref().as_ptr(),
            );
        }
    }

    // pub fn add_translation_unit_source_string_span(&self, translation_unit_index: TranslationUnitIndex, cstr path, cstr sourceBegin, cstr sourceEnd) {}
    // pub fn add_translation_unit_source_blob(&self, translation_unit_index: TranslationUnitIndex, cstr path, ISlangBlob* sourceBlob) {}

    pub fn add_entry_point(
        &self,
        translation_unit_index: TranslationUnitIndex,
        name: impl AsRef<CStr>,
        stage: Stage,
    ) -> EntryPointIndex {
        unsafe {
            spAddEntryPoint(
                self.get(),
                translation_unit_index.get(),
                name.as_ref().as_ptr(),
                stage as SlangStage,
            )
            .into()
        }
    }

    pub fn add_entry_point_ex(
        &self,
        translation_unit_index: TranslationUnitIndex,
        name: impl AsRef<CStr>,
        stage: Stage,
        generic_type_names: &[&CStr],
    ) -> EntryPointIndex {
        unsafe {
            let mut generic_type_names_ptrs = generic_type_names
                .iter()
                .map(|a| a.as_ptr())
                .collect::<Vec<_>>();

            dbg!(&generic_type_names);
            dbg!(&generic_type_names_ptrs);

            spAddEntryPointEx(
                self.get(),
                translation_unit_index.get(),
                name.as_ref().as_ptr(),
                stage as SlangStage,
                generic_type_names.len() as i32,
                &mut generic_type_names_ptrs[0],
            )
            .into()
        }
    }

    pub fn compile(&self) -> Result<()> {
        unsafe { into_result(spCompile(self.get())) }
    }

    pub fn get_diagnostic_output(&self) -> &'a CStr {
        unsafe { CStr::from_ptr(spGetDiagnosticOutput(self.get())) }
    }

    // pub fn get_diagnostic_output_blob(&self, ISlangBlob** outBlob) -> SlangResult  {}
    pub fn get_dependency_file_count(&self) -> usize {
        unsafe { spGetDependencyFileCount(self.get()) as usize }
    }

    pub fn get_dependency_file_path(&self, index: usize) -> &'a CStr {
        unsafe { CStr::from_ptr(spGetDependencyFilePath(self.get(), index as i32)) }
    }

    pub fn get_translation_unit_count(&self) -> usize {
        unsafe { spGetTranslationUnitCount(self.get()) as usize }
    }

    pub fn get_entry_point_source(&self, entry_point_index: EntryPointIndex) -> &'a CStr {
        unsafe { CStr::from_ptr(spGetEntryPointSource(self.get(), entry_point_index.get())) }
    }

    pub fn get_entry_point_code(&self, entry_point_index: EntryPointIndex) -> &'a [u8] {
        unsafe {
            let mut out_size: usize = 0;
            let blob = spGetEntryPointCode(
                self.get(),
                entry_point_index.get(),
                &mut out_size as *mut usize,
            );

            std::slice::from_raw_parts(blob as *const u8, out_size)
        }
    }

    // pub fn get_entry_point_code_blob(&self,  entry_point_index: EntryPointIndex, targetIndex: CodeGenTarget, ISlangBlob** outBlob) -> SlangResult  {}
    // pub fn get_compile_request_code(&self, size_t* outSize) -> void const*  {}
    // pub fn get_reflection(&self) -> SlangReflection*  {}
}

impl Drop for CompileRequest<'_> {
    fn drop(&mut self) {
        unsafe {
            spDestroyCompileRequest(self.get());
        }
    }
}

type ProfileId = NonZeroU32;
