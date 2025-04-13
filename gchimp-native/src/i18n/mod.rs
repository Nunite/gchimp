use std::collections::HashMap;
use lazy_static::lazy_static;

// 支持的语言类型
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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
    // Map2Mdl
    Map,
    Entity,
    Clear,
    Options,
    AutoPickupWads,
    ExportTextures,
    UppercaseTexture,
    OnlyConvertMarked,
    CenterModel,
    Flatshade,
    ReverseNormal,
    Run,
    // BLBH
    SMD,
    Texture,
    Add,
    ConvertTexture,
    ConvertSMD,
    CompileMDL,
    // S2G
    File,
    Folder,
    Steps,
    Decompile,
    VTF,
    BMP,
    SmdQc,
    GoldSrcCompile,
    Force,
    AddSuffix,
    IgnoreConverted,
    Cancel,
    // SkyMod
    Reset,
    TexturePerFace,
    SkyboxSize,
    OutputName,
    // TexTile
    ImageExtensions,
    Tiling,
    Transparent,
    ChangeFileName,
    AddFiles,
    AddFolders,
    ListOfItems,
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
        // Map2Mdl
        en.insert(TextKey::Map, "Map:");
        en.insert(TextKey::Entity, "Entity");
        en.insert(TextKey::Clear, "Clear");
        en.insert(TextKey::Options, "Options:");
        en.insert(TextKey::AutoPickupWads, "Auto pickup WADs");
        en.insert(TextKey::ExportTextures, "Export textures");
        en.insert(TextKey::UppercaseTexture, "Uppercase texture");
        en.insert(TextKey::OnlyConvertMarked, "Only convert marked entity");
        en.insert(TextKey::CenterModel, "Center the model");
        en.insert(TextKey::Flatshade, "Flatshade");
        en.insert(TextKey::ReverseNormal, "Reverse normal");
        en.insert(TextKey::Run, "Run");
        // BLBH
        en.insert(TextKey::SMD, "SMD:");
        en.insert(TextKey::Texture, "Texture:");
        en.insert(TextKey::Add, "Add");
        en.insert(TextKey::ConvertTexture, "Convert texture");
        en.insert(TextKey::ConvertSMD, "Convert SMD");
        en.insert(TextKey::CompileMDL, "Compile MDL");
        // S2G
        en.insert(TextKey::File, "File:");
        en.insert(TextKey::Folder, "Folder:");
        en.insert(TextKey::Steps, "Steps:");
        en.insert(TextKey::Decompile, "Decompile");
        en.insert(TextKey::VTF, "VTF");
        en.insert(TextKey::BMP, "BMP");
        en.insert(TextKey::SmdQc, "Smd/Qc");
        en.insert(TextKey::GoldSrcCompile, "GoldSrc Compile");
        en.insert(TextKey::Force, "Force");
        en.insert(TextKey::AddSuffix, "Add suffix");
        en.insert(TextKey::IgnoreConverted, "Ignore converted");
        en.insert(TextKey::Cancel, "Cancel");
        // SkyMod
        en.insert(TextKey::Reset, "Reset");
        en.insert(TextKey::TexturePerFace, "Texture per face:");
        en.insert(TextKey::SkyboxSize, "Skybox size:");
        en.insert(TextKey::OutputName, "Output name:");
        // TexTile
        en.insert(TextKey::ImageExtensions, "Image extensions");
        en.insert(TextKey::Tiling, "Tiling");
        en.insert(TextKey::Transparent, "Transparent");
        en.insert(TextKey::ChangeFileName, "Change file name");
        en.insert(TextKey::AddFiles, "Add file(s)");
        en.insert(TextKey::AddFolders, "Add folder(s)");
        en.insert(TextKey::ListOfItems, "List of items");
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
        // Map2Mdl
        zh.insert(TextKey::Map, "地图:");
        zh.insert(TextKey::Entity, "实体");
        zh.insert(TextKey::Clear, "清除");
        zh.insert(TextKey::Options, "选项:");
        zh.insert(TextKey::AutoPickupWads, "自动获取WAD");
        zh.insert(TextKey::ExportTextures, "导出纹理");
        zh.insert(TextKey::UppercaseTexture, "大写纹理");
        zh.insert(TextKey::OnlyConvertMarked, "仅转换标记的实体");
        zh.insert(TextKey::CenterModel, "居中模型");
        zh.insert(TextKey::Flatshade, "平面着色");
        zh.insert(TextKey::ReverseNormal, "反转法线");
        zh.insert(TextKey::Run, "运行");
        // BLBH
        zh.insert(TextKey::SMD, "SMD:");
        zh.insert(TextKey::Texture, "纹理:");
        zh.insert(TextKey::Add, "添加");
        zh.insert(TextKey::ConvertTexture, "转换纹理");
        zh.insert(TextKey::ConvertSMD, "转换SMD");
        zh.insert(TextKey::CompileMDL, "编译MDL");
        // S2G
        zh.insert(TextKey::File, "文件:");
        zh.insert(TextKey::Folder, "文件夹:");
        zh.insert(TextKey::Steps, "步骤:");
        zh.insert(TextKey::Decompile, "反编译");
        zh.insert(TextKey::VTF, "VTF");
        zh.insert(TextKey::BMP, "BMP");
        zh.insert(TextKey::SmdQc, "Smd/Qc");
        zh.insert(TextKey::GoldSrcCompile, "GoldSrc编译");
        zh.insert(TextKey::Force, "强制");
        zh.insert(TextKey::AddSuffix, "添加后缀");
        zh.insert(TextKey::IgnoreConverted, "忽略已转换");
        zh.insert(TextKey::Cancel, "取消");
        // SkyMod
        zh.insert(TextKey::Reset, "重置");
        zh.insert(TextKey::TexturePerFace, "每面纹理数:");
        zh.insert(TextKey::SkyboxSize, "天空盒大小:");
        zh.insert(TextKey::OutputName, "输出名称:");
        // TexTile
        zh.insert(TextKey::ImageExtensions, "图片扩展名");
        zh.insert(TextKey::Tiling, "平铺");
        zh.insert(TextKey::Transparent, "透明");
        zh.insert(TextKey::ChangeFileName, "更改文件名");
        zh.insert(TextKey::AddFiles, "添加文件");
        zh.insert(TextKey::AddFolders, "添加文件夹");
        zh.insert(TextKey::ListOfItems, "项目列表");
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