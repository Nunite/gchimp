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
    // Status messages
    Idle,
    Running,
    Done,
    OK,
    // Hover texts
    UVClampHint,
    TexturePerFaceHint,
    SkyboxSizeHint,
    ConvertTextureHint,
    FlatshadeHint,
    FileExtensionsHint,
    TilingHint,
    TransparentHint,
    ChangeFileNameHint,
    RemoveItemHint,
    AutoPickupWadsHint,
    ExportTexturesHint,
    UppercaseTextureHint,
    OnlyConvertMarkedHint,
    CenterModelHint,
    FlatshadeModelHint,
    ReverseNormalHint,
    ConvertTextureBlbhHint,
    ConvertSmdHint,
    CompileMdlHint,
    ForceHint,
    AddSuffixHint,
    IgnoreConvertedHint,
    FlatshadeTextureHint,
    // 新增的提示文本键
    VTFHint,
    BMPHint,
    SmdQcHint,
    GoldSrcCompileHint,
    // BLBH 相关的提示文本键
    ConvertSMDHint,
    CompileMDLHint,
    // Misc 相关的提示文本键
    SplitModelTitle,
    SplitModelHint,
    LoopWaveTitle,
    LoopWaveHint,
    LoopCheckboxHint,
    ResMakeTitle,
    ResMakeHint,
    WadCheckHint,
    DefaultResHint,
    IgnoreMissingHint,
    CreateLinkedWadHint,
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
        // Status messages
        en.insert(TextKey::Idle, "Idle");
        en.insert(TextKey::Running, "Running");
        en.insert(TextKey::Done, "Done");
        en.insert(TextKey::OK, "OK");
        // Hover texts
        en.insert(TextKey::UVClampHint, "There is a problem with the edge where it is repeatedly filtered.\nWith this option, the polygon UV will not sample the edge by avoiding the edge.\nBy default, it will \"shrink\" the UV in by 1 pixel wherever applicable.");
        en.insert(TextKey::TexturePerFaceHint, "How many textures should each skybox face have?\nIt should be a perfect square (such as 1, 4, 9, 16, ..)\nIf a model has more than 64 textures, it will be split into smaller models");
        en.insert(TextKey::SkyboxSizeHint, "The size of the model");
        en.insert(TextKey::ConvertTextureHint, "Converts most image format into compliant BMP.\nProcesses textures into suitable format for other settings.\nRecommended to leave it checked.");
        en.insert(TextKey::FlatshadeHint, "Mark texture with flatshade flag.\nRecommeded to leave it checked for uniformly lit texture.");
        en.insert(TextKey::FileExtensionsHint, "Converts only textures with specified file extension(s)\nSpace seperated");
        en.insert(TextKey::TilingHint, "The dimensions of a texture will multiply by this number.");
        en.insert(TextKey::TransparentHint, "The threshold to decide whether a texture is transparent.\nIf the dominant color of an image exceeds this threshold,\nit will be chosen as transparent mask.");
        en.insert(TextKey::ChangeFileNameHint, "Prepend \"{\" if transparent\nAppend \"_<scalar>\" if tiling");
        en.insert(TextKey::RemoveItemHint, "Right click to remove");
        en.insert(TextKey::AutoPickupWadsHint, "Look for WAD files from \"wad\" key in the map file or worldbrush entity");
        en.insert(TextKey::ExportTexturesHint, "Export textures into the map file folder OR studiomdl.exe folder if converting entity");
        en.insert(TextKey::UppercaseTextureHint, "For .map exported from .jmf/rmf, the texture used inside source map file does not match WAD file.\nThis option is to coerce every texture in this process to be upper case.");
        en.insert(TextKey::OnlyConvertMarkedHint, "Only convert brush entities {} and this would modify the original map file");
        en.insert(TextKey::CenterModelHint, "The center of the model is the origin");
        en.insert(TextKey::FlatshadeModelHint, "Model is flatshade");
        en.insert(TextKey::ReverseNormalHint, "Reverses every vertex normals");
        en.insert(TextKey::ConvertTextureBlbhHint, "Splits 4096x4096 texture into 64 smaller compliant files");
        en.insert(TextKey::ConvertSmdHint, "Creates new SMD file that will use those new texture files accordingly");
        en.insert(TextKey::CompileMdlHint, "Creates QC file and compiles the model with included studiomdl.exe");
        en.insert(TextKey::ForceHint, "Continues with the process even when there is error.");
        en.insert(TextKey::AddSuffixHint, "Adds suffix \"_goldsrc\" to the name of the converted model");
        en.insert(TextKey::IgnoreConvertedHint, "Ignores models with \"_goldsrc\" suffix");
        en.insert(TextKey::FlatshadeTextureHint, "Textures will have flat shade flags\nRecommended to have it on so textures will be uniformly lit");
        en.insert(TextKey::VTFHint, "Convert textures to VTF format");
        en.insert(TextKey::BMPHint, "Convert textures to BMP format");
        en.insert(TextKey::SmdQcHint, "Generate SMD and QC files");
        en.insert(TextKey::GoldSrcCompileHint, "Compile model for GoldSrc engine");
        en.insert(TextKey::ConvertSMDHint, "Creates new SMD file that will use those new texture files accordingly");
        en.insert(TextKey::CompileMDLHint, "Creates QC file and compiles the model with included studiomdl.exe");
        en.insert(TextKey::SplitModelTitle, "Split Model Title");
        en.insert(TextKey::SplitModelHint, "Split Model Hint");
        en.insert(TextKey::LoopWaveTitle, "Loop Wave Title");
        en.insert(TextKey::LoopWaveHint, "Loop Wave Hint");
        en.insert(TextKey::LoopCheckboxHint, "Loop Checkbox Hint");
        en.insert(TextKey::ResMakeTitle, "ResMake Title");
        en.insert(TextKey::ResMakeHint, "ResMake Hint");
        en.insert(TextKey::WadCheckHint, "Wad Check Hint");
        en.insert(TextKey::DefaultResHint, "Default Res Hint");
        en.insert(TextKey::IgnoreMissingHint, "Ignore Missing Hint");
        en.insert(TextKey::CreateLinkedWadHint, "Create Linked Wad Hint");
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
        // Status messages
        zh.insert(TextKey::Idle, "空闲");
        zh.insert(TextKey::Running, "运行中");
        zh.insert(TextKey::Done, "完成");
        zh.insert(TextKey::OK, "成功");
        // Hover texts
        zh.insert(TextKey::UVClampHint, "边缘处存在重复过滤的问题。\n启用此选项后，多边形UV将通过避开边缘来避免采样边缘。\n默认情况下，它会在适用的地方将UV向内收缩1个像素。");
        zh.insert(TextKey::TexturePerFaceHint, "每个天空盒面应该有多少纹理？\n应该是一个完全平方数(如1、4、9、16等)\n如果模型有超过64个纹理，它将被分割成更小的模型");
        zh.insert(TextKey::SkyboxSizeHint, "模型的大小");
        zh.insert(TextKey::ConvertTextureHint, "将大多数图像格式转换为兼容的BMP格式。\n根据其他设置处理纹理为合适的格式。\n建议保持选中状态。");
        zh.insert(TextKey::FlatshadeHint, "为纹理标记平面着色标志。\n建议保持选中状态以获得均匀的光照纹理。");
        zh.insert(TextKey::FileExtensionsHint, "仅转换指定文件扩展名的纹理\n用空格分隔");
        zh.insert(TextKey::TilingHint, "纹理的尺寸将乘以这个数字。");
        zh.insert(TextKey::TransparentHint, "决定纹理是否透明的阈值。\n如果图像的主要颜色超过此阈值，\n它将被选为透明遮罩。");
        zh.insert(TextKey::ChangeFileNameHint, "如果透明则在前面添加 \"{\"\n如果平铺则在后面添加 \"_<倍数>\"");
        zh.insert(TextKey::RemoveItemHint, "右键点击删除");
        zh.insert(TextKey::AutoPickupWadsHint, "从地图文件或世界画笔实体的\"wad\"键中查找WAD文件");
        zh.insert(TextKey::ExportTexturesHint, "将纹理导出到地图文件夹或转换实体时导出到studiomdl.exe文件夹");
        zh.insert(TextKey::UppercaseTextureHint, "对于从.jmf/rmf导出的.map文件，源地图文件中使用的纹理与WAD文件不匹配。\n此选项将在此过程中强制所有纹理为大写。");
        zh.insert(TextKey::OnlyConvertMarkedHint, "仅转换标记的画笔实体{}，这将修改原始地图文件");
        zh.insert(TextKey::CenterModelHint, "模型的中心在原点");
        zh.insert(TextKey::FlatshadeModelHint, "模型使用平面着色");
        zh.insert(TextKey::ReverseNormalHint, "反转所有顶点法线");
        zh.insert(TextKey::ConvertTextureBlbhHint, "将4096x4096纹理分割成64个较小的兼容文件");
        zh.insert(TextKey::ConvertSmdHint, "创建新的SMD文件，使用相应的新纹理文件");
        zh.insert(TextKey::CompileMdlHint, "创建QC文件并使用包含的studiomdl.exe编译模型");
        zh.insert(TextKey::ForceHint, "即使出现错误也继续处理。");
        zh.insert(TextKey::AddSuffixHint, "为转换后的模型名称添加\"_goldsrc\"后缀");
        zh.insert(TextKey::IgnoreConvertedHint, "忽略带有\"_goldsrc\"后缀的模型");
        zh.insert(TextKey::FlatshadeTextureHint, "纹理将具有平面着色标志\n建议开启以使纹理均匀照明");
        zh.insert(TextKey::VTFHint, "将纹理转换为VTF格式");
        zh.insert(TextKey::BMPHint, "将纹理转换为BMP格式");
        zh.insert(TextKey::SmdQcHint, "生成SMD和QC文件");
        zh.insert(TextKey::GoldSrcCompileHint, "为GoldSrc引擎编译模型");
        zh.insert(TextKey::ConvertSMDHint, "创建新的SMD文件，使用相应的新纹理文件");
        zh.insert(TextKey::CompileMDLHint, "创建QC文件并使用包含的studiomdl.exe编译模型");
        zh.insert(TextKey::SplitModelTitle, "分割模型标题");
        zh.insert(TextKey::SplitModelHint, "分割模型提示");
        zh.insert(TextKey::LoopWaveTitle, "循环波形标题");
        zh.insert(TextKey::LoopWaveHint, "循环波形提示");
        zh.insert(TextKey::LoopCheckboxHint, "循环复选框提示");
        zh.insert(TextKey::ResMakeTitle, "ResMake标题");
        zh.insert(TextKey::ResMakeHint, "ResMake提示");
        zh.insert(TextKey::WadCheckHint, "Wad检查提示");
        zh.insert(TextKey::DefaultResHint, "默认Res提示");
        zh.insert(TextKey::IgnoreMissingHint, "忽略缺失提示");
        zh.insert(TextKey::CreateLinkedWadHint, "创建链接Wad提示");
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