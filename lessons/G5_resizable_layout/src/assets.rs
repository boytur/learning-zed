use gpui::{AssetSource, Result, SharedString};
use std::borrow::Cow;
use std::fs;
use std::path::Path;

pub struct LocalAssets;

impl AssetSource for LocalAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        let full_path = Path::new("lessons/G5_resizable_layout").join(path);
        let bytes = fs::read(&full_path)?;
        Ok(Some(Cow::Owned(bytes)))
    }

    fn list(&self, _path: &str) -> Result<Vec<SharedString>> {
        Ok(vec![])
    }
}
