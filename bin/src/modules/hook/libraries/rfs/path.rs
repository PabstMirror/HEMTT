use rhai::plugin::{
    export_module, mem, Dynamic, FnAccess, FnNamespace, ImmutableString, Module, NativeCallContext,
    PluginFunction, RhaiResult, TypeId,
};

#[allow(clippy::needless_pass_by_ref_mut)]
#[allow(clippy::ptr_arg)]
#[export_module]
pub mod path_functions {
    use std::path::PathBuf;

    #[rhai_fn(global, pure)]
    pub fn join(path: &mut PathBuf, other: &str) -> PathBuf {
        path.join(other)
    }

    #[rhai_fn(global, pure)]
    pub fn exists(path: &mut PathBuf) -> bool {
        path.exists()
    }

    #[rhai_fn(global, pure)]
    pub fn is_dir(path: &mut PathBuf) -> bool {
        path.is_dir()
    }

    #[rhai_fn(global, pure)]
    pub fn is_file(path: &mut PathBuf) -> bool {
        path.is_file()
    }

    #[rhai_fn(global, pure)]
    pub fn parent(path: &mut PathBuf) -> PathBuf {
        path.parent().unwrap().to_path_buf()
    }

    #[rhai_fn(global, pure)]
    pub fn file_name(path: &mut PathBuf) -> String {
        path.file_name().unwrap().to_str().unwrap().to_string()
    }

    #[rhai_fn(global, name = "to_string", name = "to_debug", pure)]
    pub fn to_string(path: &mut PathBuf) -> String {
        path.display().to_string().replace('\\', "/")
    }

    #[rhai_fn(global, name = "copy", pure)]
    pub fn copy(path: &mut PathBuf, other: PathBuf) -> bool {
        if path.is_dir() {
            fs_extra::dir::copy(path, other, &fs_extra::dir::CopyOptions::new()).is_ok()
        } else {
            std::fs::copy(path, other).is_ok()
        }
    }

    #[rhai_fn(global, name = "move", pure)]
    pub fn _move(path: &mut PathBuf, other: PathBuf) -> bool {
        if path.is_dir() {
            fs_extra::dir::move_dir(path, other, &fs_extra::dir::CopyOptions::new()).is_ok()
        } else {
            std::fs::rename(path, other).is_ok()
        }
    }

    #[rhai_fn(global, pure)]
    pub fn list(path: &mut PathBuf) -> rhai::Array {
        let mut list = Vec::new();
        if path.is_dir() {
            for entry in std::fs::read_dir(path).expect("can't read dir") {
                let entry = entry.expect("entry failed");
                list.push(Dynamic::from(entry.path()));
            }
        }
        list
    }
}
