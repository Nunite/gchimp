use std::collections::HashMap;
use lazy_static::lazy_static;

// 支持的语言类型
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    English,
    Chinese,
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

// 语言文本键值
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextKey {
    MenuNew,
    MenuOpen,
    MenuOpenRecent,
    MenuSave,
    MenuSaveAs,
    MenuFind,
    MenuImport,
    MenuExportAll,
    MenuOptions,
    MenuClose,
    DragDropHint,
    NoRecentFiles,
    FitTexture,
    ToUppercase,
    ToLowercase,
}

lazy_static! {
    static ref TRANSLATIONS: HashMap<Language, HashMap<TextKey, &'static str>> = {
        let mut m = HashMap::new();
        
        // English translations
        let mut en = HashMap::new();
        en.insert(TextKey::MenuNew, "New");
        en.insert(TextKey::MenuOpen, "Open");
        en.insert(TextKey::MenuOpenRecent, "Open Recent");
        en.insert(TextKey::MenuSave, "Save (Ctrl+S)");
        en.insert(TextKey::MenuSaveAs, "Save As");
        en.insert(TextKey::MenuFind, "Find (Ctrl+F)");
        en.insert(TextKey::MenuImport, "Import");
        en.insert(TextKey::MenuExportAll, "Export All");
        en.insert(TextKey::MenuOptions, "Options");
        en.insert(TextKey::MenuClose, "Close");
        en.insert(TextKey::DragDropHint, "Drag and drop a WAD file to start.\nYou can also drag and drop a BSP file.");
        en.insert(TextKey::NoRecentFiles, "No recently opened");
        en.insert(TextKey::FitTexture, "Fit texture");
        en.insert(TextKey::ToUppercase, "To UPPERCASE");
        en.insert(TextKey::ToLowercase, "To lowercase");
        m.insert(Language::English, en);

        // Chinese translations
        let mut zh = HashMap::new();
        zh.insert(TextKey::MenuNew, "新建");
        zh.insert(TextKey::MenuOpen, "打开");
        zh.insert(TextKey::MenuOpenRecent, "最近打开");
        zh.insert(TextKey::MenuSave, "保存 (Ctrl+S)");
        zh.insert(TextKey::MenuSaveAs, "另存为");
        zh.insert(TextKey::MenuFind, "查找 (Ctrl+F)");
        zh.insert(TextKey::MenuImport, "导入");
        zh.insert(TextKey::MenuExportAll, "导出全部");
        zh.insert(TextKey::MenuOptions, "选项");
        zh.insert(TextKey::MenuClose, "关闭");
        zh.insert(TextKey::DragDropHint, "拖放一个WAD文件来开始。\n你也可以拖放一个BSP文件。");
        zh.insert(TextKey::NoRecentFiles, "没有最近打开的文件");
        zh.insert(TextKey::FitTexture, "适应纹理");
        zh.insert(TextKey::ToUppercase, "转为大写");
        zh.insert(TextKey::ToLowercase, "转为小写");
        m.insert(Language::Chinese, zh);

        m
    };
}

pub fn get_text(key: TextKey, lang: Language) -> &'static str {
    TRANSLATIONS
        .get(&lang)
        .and_then(|translations| translations.get(&key))
        .map_or("Missing translation", |v| v)
} 