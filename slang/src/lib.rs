use slang_sys::*;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::num::NonZeroU32;
use std::ptr;

pub use slang_sys as ffi;

// TODO impl Display for SlangResult wrapper? can parse severity, facility, code
type Result<T> = ::std::result::Result<T, SlangResult>;

#[derive(Debug)]
pub struct Session<'a> {
    handle: *mut SlangSession,
    phantom: PhantomData<&'a SlangSession>,
}

impl Session<'_> {
    pub fn new() -> Self {
        Default::default()
    }

    fn set_shared_library_loader(&self, loader: ISlangSharedLibraryLoader) {
        unsafe { unimplemented!() }
    }

    fn get_shared_library_loader(&self) -> ISlangSharedLibraryLoader {
        unsafe { unimplemented!() }
    }

    // bool?
    fn check_compile_target_support(&self, target: SlangCompileTarget) -> Result<()> {
        unsafe { unimplemented!() }
    }

    // bool?
    fn check_pass_through_support(&self, pass_through: SlangPassThrough) -> Result<()> {
        unsafe { unimplemented!() }
    }

    fn add_builtins(&self, source_path: impl AsRef<CStr>, source_string: impl AsRef<CStr>) {
        unsafe { unimplemented!() }
    }

    fn create_compile_request(&self) -> CompileRequest {
        unsafe {
            CompileRequest {
                handle: spCreateCompileRequest(self.handle),
                phantom: PhantomData,
            }
        }
    }

    fn find_profile(&self, name: impl AsRef<CStr>) -> Profile {
        unsafe {
            // For whatever reason bindgen generates enums as i32.
            const SLANG_PROFILE_UNKNOWN_AS_U32: u32 = SLANG_PROFILE_UNKNOWN as u32;

            match spFindProfile(self.handle, name.as_ref().as_ptr()) {
                SLANG_PROFILE_UNKNOWN_AS_U32 => Profile::Unknown,
                id => Profile::Id(
                    NonZeroU32::new(id).expect("SlangProfileID should non-zero in the valid case."),
                ),
            }
        }
    }
}

impl Default for Session<'_> {
    fn default() -> Self {
        unsafe {
            Session {
                handle: spCreateSession(ptr::null()),
                phantom: PhantomData,
            }
        }
    }
}

impl Drop for Session<'_> {
    fn drop(&mut self) {
        unsafe {
            spDestroySession(self.handle);
        }
    }
}

pub struct CompileRequest<'a> {
    handle: *mut SlangCompileRequest,
    phantom: PhantomData<&'a SlangCompileRequest>,
}

// TODO lifetime to compile request?
type CodeGenTarget = i32;
type TranslationUnitIndex = i32;
type EntryPointIndex = i32;

impl<'a> CompileRequest<'a> {
    // fn set_file_system(&self, ISlangFileSystem* fileSystem) {}
    // fn set_compile_flags(&self, SlangCompileFlags flags) {}
    // fn set_dump_intermediates(&self, int enable) {}
    // fn set_line_directive_mode(&self, SlangLineDirectiveMode mode) {}
    // fn set_code_gen_target(&self, SlangCompileTarget target) {}

    fn add_code_gen_target(&self, target: SlangCompileTarget) -> CodeGenTarget {
        unimplemented!()
    }

    fn set_target_profile(&self, targetIndex: CodeGenTarget, profile: ProfileId) {
        unimplemented!()
    }

    // fn set_target_flags(&self, targetIndex: CodeGenTarget, SlangTargetFlags flags) {}
    // fn set_target_floating_point_mode(&self, targetIndex: CodeGenTarget, SlangFloatingPointMode mode) {}
    // fn set_target_matrix_layout_mode(&self, targetIndex: CodeGenTarget, SlangMatrixLayoutMode mode) {}
    // fn set_matrix_layout_mode(&self, SlangMatrixLayoutMode mode) {}
    // fn set_output_container_format(&self, SlangContainerFormat format) {}
    // fn set_pass_through(&self, SlangPassThrough passThrough) {}
    // fn set_diagnostic_callback(&self, SlangDiagnosticCallback callback, void const* userData) {}
    // fn set_writer(&self, SlangWriterChannel channel, ISlangWriter* writer) {}
    // fn get_writer(&self, SlangWriterChannel channel) -> ISlangWriter*  {}
    // fn add_search_path(&self, const char* searchDir) {}
    // fn add_preprocessor_define(&self, const char* key, const char* value) {}
    // fn process_command_line_arguments(&self, cstr const* args, int argCount) -> SlangResult  {}

    fn add_translation_unit(
        &self,
        language: SlangSourceLanguage,
        name: impl AsRef<CStr>,
    ) -> TranslationUnitIndex {
        unimplemented!()
    }

    // fn translation_unit_add_preprocessor_define(&self, translationUnitIndex: TranslationUnitIndex, const char* key, const char* value) {}
    // fn add_translation_unit_source_file(&self, translationUnitIndex: TranslationUnitIndex, cstr path) {}

    fn add_translation_unit_source_string(
        &self,
        translationUnitIndex: TranslationUnitIndex,
        path: impl AsRef<CStr>,
        source: impl AsRef<CStr>,
    ) {
        unimplemented!()
    }

    // fn add_translation_unit_source_string_span(&self, translationUnitIndex: TranslationUnitIndex, cstr path, cstr sourceBegin, cstr sourceEnd) {}
    // fn add_translation_unit_source_blob(&self, translationUnitIndex: TranslationUnitIndex, cstr path, ISlangBlob* sourceBlob) {}

    fn add_entry_point(
        &self,
        translationUnitIndex: TranslationUnitIndex,
        name: impl AsRef<CStr>,
        stage: SlangStage,
    ) -> EntryPointIndex {
        unimplemented!()
    }

    // fn add_entry_point_ex(&self, translationUnitIndex: TranslationUnitIndex, cstr name, SlangStage stage, int genericTypeNameCount, cstr* genericTypeNames) -> int  {}

    fn compile(&self) -> SlangResult {
        unsafe { spCompile(self.handle) }
    }

    fn get_diagnostic_output(&self) -> &'a CStr {
        unimplemented!()
    }

    // fn get_diagnostic_output_blob(&self, ISlangBlob** outBlob) -> SlangResult  {}
    // fn get_dependency_file_count(&self) -> int  {}
    // fn get_dependency_file_path(&self, int index) -> cstr  {}
    // fn get_translation_unit_count(&self) -> int  {}
    // fn get_entry_point_source(&self,  entryPointIndex: EntryPointIndex) -> cstr  {}

    fn get_entry_point_code(&self, entryPointIndex: EntryPointIndex) -> &'a [u8] {
        unimplemented!()
    }

    // fn get_entry_point_code_blob(&self,  entryPointIndex: EntryPointIndex, targetIndex: CodeGenTarget, ISlangBlob** outBlob) -> SlangResult  {}
    // fn get_compile_request_code(&self, size_t* outSize) -> void const*  {}
    // fn get_reflection(&self) -> SlangReflection*  {}
}

impl Drop for CompileRequest<'_> {
    fn drop(&mut self) {
        unsafe {
            spDestroyCompileRequest(self.handle);
        }
    }
}

type ProfileId = NonZeroU32;

pub enum Profile {
    Id(ProfileId),
    Unknown,
}
