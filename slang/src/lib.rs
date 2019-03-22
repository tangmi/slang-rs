#![allow(dead_code)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use slang_sys::*;
use std::ffi::CStr;
use std::num::NonZeroU32;
use std::ptr;

mod result;

pub use slang_sys as ffi;

use result::into_result;
use result::Result;

/// Implementation details for `handle_wrapper_struct`.
macro_rules! handle_wrapper_struct_impl {
    ($wrapper_name:ident, $inner:ty) => {
        impl ::std::fmt::Debug for $wrapper_name<'_> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.debug_tuple(stringify!($wrapper_name))
                    .field(&self.handle)
                    .finish()
            }
        }

        impl<'a> $wrapper_name<'a> {
            #[inline]
            pub fn wrap(inner: $inner) -> Self {
                Self {
                    handle: inner,
                    phantom: ::std::marker::PhantomData,
                }
            }

            #[inline]
            pub fn get(&self) -> $inner {
                self.handle
            }
        }

        impl From<$inner> for $wrapper_name<'_> {
            fn from(inner: $inner) -> Self {
                Self::wrap(inner)
            }
        }
    };
}

/// Generate a wrapper struct with a lifetime specifier.
///
/// Use `$wrapper_name::wrap($inner)` and `$wrapper_name::get() -> $inner` to wrap and access the value.
macro_rules! handle_wrapper_struct {
    ($wrapper_name:ident, $inner:ty) => {
        pub struct $wrapper_name<'a> {
            handle: $inner,
            phantom: ::std::marker::PhantomData<&'a $inner>,
        }
        handle_wrapper_struct_impl!($wrapper_name, $inner);
    };
}

/// `handle_wrapper_struct` for `Copy` types.
macro_rules! handle_wrapper_struct_copy {
    ($wrapper_name:ident, $inner:ty) => {
        #[derive(Copy, Clone)]
        pub struct $wrapper_name<'a> {
            handle: $inner,
            phantom: ::std::marker::PhantomData<&'a $inner>,
        }
        handle_wrapper_struct_impl!($wrapper_name, $inner);
    };
}

handle_wrapper_struct!(Session, *mut SlangSession);

impl Default for Session<'_> {
    fn default() -> Self {
        unsafe { spCreateSession(ptr::null()).into() }
    }
}

impl Session<'_> {
    pub fn new() -> Self {
        Self::default()
    }

    // fn set_shared_library_loader(&self, loader: ISlangSharedLibraryLoader) {
    //     unimplemented!()
    // }

    // fn get_shared_library_loader(&self) -> ISlangSharedLibraryLoader {
    //     unimplemented!()
    // }

    // bool?
    fn check_compile_target_support(&self, target: SlangCompileTarget) -> Result {
        unsafe { into_result(spSessionCheckCompileTargetSupport(self.get(), target)) }
    }

    // bool?
    fn check_pass_through_support(&self, pass_through: SlangPassThrough) -> Result {
        unsafe { into_result(spSessionCheckPassThroughSupport(self.get(), pass_through)) }
    }

    fn add_builtins(&self, source_path: impl AsRef<CStr>, source_string: impl AsRef<CStr>) {
        unsafe {
            spAddBuiltins(
                self.get(),
                source_path.as_ref().as_ptr(),
                source_string.as_ref().as_ptr(),
            )
        }
    }

    fn create_compile_request(&self) -> CompileRequest {
        unsafe { spCreateCompileRequest(self.get()).into() }
    }

    fn find_profile(&self, name: impl AsRef<CStr>) -> Option<ProfileId> {
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

handle_wrapper_struct_copy!(CodeGenTarget, i32);
handle_wrapper_struct_copy!(TranslationUnitIndex, i32);
handle_wrapper_struct_copy!(EntryPointIndex, i32);

handle_wrapper_struct!(CompileRequest, *mut SlangCompileRequest);

impl<'a> CompileRequest<'a> {
    // fn set_file_system(&self, ISlangFileSystem* fileSystem) {}
    // fn set_compile_flags(&self, SlangCompileFlags flags) {}
    // fn set_dump_intermediates(&self, int enable) {}
    // fn set_line_directive_mode(&self, SlangLineDirectiveMode mode) {}
    // fn set_code_gen_target(&self, SlangCompileTarget target) {}

    fn add_code_gen_target(&self, target: SlangCompileTarget) -> CodeGenTarget {
        unsafe { spAddCodeGenTarget(self.get(), target).into() }
    }

    fn set_target_profile(&self, target_index: CodeGenTarget, profile: ProfileId) {
        unsafe { spSetTargetProfile(self.get(), target_index.get(), profile.get()) }
    }

    // fn set_target_flags(&self, target_index: CodeGenTarget, SlangTargetFlags flags) {}
    // fn set_target_floating_point_mode(&self, target_index: CodeGenTarget, SlangFloatingPointMode mode) {}
    // fn set_target_matrix_layout_mode(&self, target_index: CodeGenTarget, SlangMatrixLayoutMode mode) {}
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
        unsafe { spAddTranslationUnit(self.get(), language, name.as_ref().as_ptr()).into() }
    }

    // fn translation_unit_add_preprocessor_define(&self, translation_unit_index: TranslationUnitIndex, const char* key, const char* value) {}
    // fn add_translation_unit_source_file(&self, translation_unit_index: TranslationUnitIndex, cstr path) {}

    fn add_translation_unit_source_string(
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
            )
        }
    }

    // fn add_translation_unit_source_string_span(&self, translation_unit_index: TranslationUnitIndex, cstr path, cstr sourceBegin, cstr sourceEnd) {}
    // fn add_translation_unit_source_blob(&self, translation_unit_index: TranslationUnitIndex, cstr path, ISlangBlob* sourceBlob) {}

    fn add_entry_point(
        &self,
        translation_unit_index: TranslationUnitIndex,
        name: impl AsRef<CStr>,
        stage: SlangStage,
    ) -> EntryPointIndex {
        unsafe {
            spAddEntryPoint(
                self.get(),
                translation_unit_index.get(),
                name.as_ref().as_ptr(),
                stage,
            )
            .into()
        }
    }

    // fn add_entry_point_ex(&self, translation_unit_index: TranslationUnitIndex, cstr name, SlangStage stage, int genericTypeNameCount, cstr* genericTypeNames) -> int  {}

    fn compile(&self) -> Result {
        unsafe { into_result(spCompile(self.get())) }
    }

    fn get_diagnostic_output(&self) -> &'a CStr {
        unsafe { CStr::from_ptr(spGetDiagnosticOutput(self.get())) }
    }

    // fn get_diagnostic_output_blob(&self, ISlangBlob** outBlob) -> SlangResult  {}
    // fn get_dependency_file_count(&self) -> int  {}
    // fn get_dependency_file_path(&self, int index) -> cstr  {}
    // fn get_translation_unit_count(&self) -> int  {}
    // fn get_entry_point_source(&self,  entry_point_index: EntryPointIndex) -> cstr  {}

    fn get_entry_point_code(&self, entry_point_index: EntryPointIndex) -> &'a [u8] {
        unsafe {
            let out_size = std::ptr::null_mut();
            let blob = spGetEntryPointCode(self.get(), entry_point_index.get(), out_size);

            std::slice::from_raw_parts(blob as *const u8, *out_size)
        }
    }

    // fn get_entry_point_code_blob(&self,  entry_point_index: EntryPointIndex, targetIndex: CodeGenTarget, ISlangBlob** outBlob) -> SlangResult  {}
    // fn get_compile_request_code(&self, size_t* outSize) -> void const*  {}
    // fn get_reflection(&self) -> SlangReflection*  {}
}

impl Drop for CompileRequest<'_> {
    fn drop(&mut self) {
        unsafe {
            spDestroyCompileRequest(self.get());
        }
    }
}

type ProfileId = NonZeroU32;
