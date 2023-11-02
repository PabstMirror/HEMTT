use rhai::plugin::{
    export_module, mem, Dynamic, FnAccess, FnNamespace, ImmutableString, Module, NativeCallContext,
    PluginFunction, RhaiResult, TypeId,
};

#[allow(clippy::needless_pass_by_ref_mut)]
#[export_module]
pub mod path_functions {
    use rhai::{Array, EvalAltResult};
    use vfs::VfsPath;

    #[rhai_fn(global, pure, return_raw)]
    pub fn join(path: &mut VfsPath, other: &str) -> Result<VfsPath, Box<EvalAltResult>> {
        path.join(other).map_err(|e| e.to_string().into())
    }

    #[rhai_fn(global, pure, return_raw)]
    pub fn exists(path: &mut VfsPath) -> Result<bool, Box<EvalAltResult>> {
        path.exists().map_err(|e| e.to_string().into())
    }

    #[rhai_fn(global, pure, return_raw)]
    pub fn is_dir(path: &mut VfsPath) -> Result<bool, Box<EvalAltResult>> {
        path.is_dir().map_err(|e| e.to_string().into())
    }

    #[rhai_fn(global, pure, return_raw)]
    pub fn is_file(path: &mut VfsPath) -> Result<bool, Box<EvalAltResult>> {
        path.is_file().map_err(|e| e.to_string().into())
    }

    #[rhai_fn(global, pure)]
    pub fn parent(path: &mut VfsPath) -> VfsPath {
        path.parent()
    }

    #[rhai_fn(global, pure)]
    pub fn file_name(path: &mut VfsPath) -> String {
        path.filename()
    }

    #[rhai_fn(global, name = "to_string", name = "to_debug", pure)]
    pub fn to_string(path: &mut VfsPath) -> String {
        path.as_str().to_string().replace('\\', "/")
    }

    #[rhai_fn(global, return_raw)]
    pub fn copy(path: &mut VfsPath, other: VfsPath) -> Result<bool, Box<EvalAltResult>> {
        let res = if path.is_dir().map_err(|e| e.to_string())? {
            path.copy_dir(&other)
                .map_err(|e| e.to_string().into())
                .err()
        } else {
            path.copy_file(&other)
                .map_err(|e| e.to_string().into())
                .err()
        };
        res.map_or_else(|| Ok(true), Err)
    }

    #[rhai_fn(global, name = "move", return_raw)]
    pub fn _move(path: &mut VfsPath, other: VfsPath) -> Result<bool, Box<EvalAltResult>> {
        let res = if path.is_dir().map_err(|e| e.to_string())? {
            path.move_dir(&other)
                .map_err(|e| e.to_string().into())
                .err()
        } else {
            path.move_file(&other)
                .map_err(|e| e.to_string().into())
                .err()
        };
        res.map_or_else(|| Ok(true), Err)
    }

    #[rhai_fn(global, pure, return_raw)]
    pub fn list(path: &mut VfsPath) -> Result<Array, Box<EvalAltResult>> {
        let mut list = Vec::new();
        if path.is_dir().map_err(|e| e.to_string())? {
            for entry in path.read_dir().unwrap() {
                list.push(Dynamic::from(entry));
            }
        }
        Ok(list)
    }
}
