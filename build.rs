// Copyright (C) 2025 worksoup <https://github.com/worksoup/>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! 该文档为AI生成。
//!
//! 构建脚本
//!
//! 负责处理应用程序资源，包括图标生成和资源嵌入
//! 主要功能：
//! 1. 检测目标平台是否需要图标
//! 2. 从SVG生成多尺寸ICO图标
//! 3. 将资源嵌入到最终可执行文件中

use ico::{IconDirEntry, IconImage};
use resvg::{
    tiny_skia::Pixmap,
    usvg::{Options, Transform, Tree},
};
use sha2::{Digest, Sha256};
use std::{env, fs, io, path::PathBuf};
/// 该文档为AI生成。
///
/// 检查目标平台是否需要图标
///
/// # 返回
/// 如果目标平台是Windows则返回true，否则返回false
fn need_icon() -> bool {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").expect("无法获取目标操作系统信息");
    // 仅Windows平台需要图标
    matches!(target_os.as_str(), "windows")
}
/// 该文档为AI生成。
///
/// 计算文件的SHA256哈希值
///
/// # 参数
/// - `path`: 文件路径
///
/// # 返回
/// 文件的十六进制哈希字符串
fn file_hash(path: &str) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut hasher = Sha256::new();
    io::copy(&mut file, &mut hasher)?;
    let result = hasher.finalize();
    Ok(format!("{result:x}"))
}
/// 该文档为AI生成。
///
/// 从SVG文件生成ICO图标
///
/// # 参数
/// - `svg_path`: SVG源文件路径
/// - `ico_path`: 生成的ICO文件路径
fn generate_ico(svg_path: &str, ico_path: &PathBuf) -> io::Result<()> {
    const SVG_SIZE: f32 = 64.0;
    const ICO_SIZE: [u32; 6] = [16, 32, 48, 64, 128, 256];

    // 读取SVG文件
    let svg_data = fs::read(svg_path)?;
    let tree = Tree::from_data(&svg_data, &Options::default())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    // 创建图标目录
    let mut icon = ico::IconDir::new(ico::ResourceType::Icon);

    // 生成多种尺寸的图标
    for &size in &ICO_SIZE {
        let mut pixmap =
            Pixmap::new(size, size).ok_or_else(|| io::Error::other("创建像素图失败"))?;

        let scale = size as f32 / SVG_SIZE;
        resvg::render(
            &tree,
            Transform::from_row(scale, 0.0, 0.0, scale, 0.0, 0.0),
            &mut pixmap.as_mut(),
        );

        // 调试用：保存PNG文件
        // pixmap
        //     .save_png(format!("res/logo_{0}x{0}.png", size))
        //     .unwrap();
        // 编码为PNG并添加到图标目录
        let png_data = pixmap.encode_png().map_err(io::Error::other)?;
        let image = IconImage::read_png(&*png_data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        icon.add_entry(
            IconDirEntry::encode_as_png(&image)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
        );
    }

    // 写入ICO文件
    let ico_file = fs::File::create(ico_path)?;
    icon.write(ico_file)?;
    Ok(())
}
/// 该文档为AI生成。
///
/// 资源处理函数
///
/// 主要功能：
/// 1. 设置文件变更监控
/// 2. 检查目标平台是否需要图标
/// 3. 生成ICO图标文件（如果不存在）
/// 4. 编译资源文件
fn handle_resource() -> io::Result<()> {
    // 资源文件路径常量
    const LOGO_RC: &str = "res/logo.rc";
    const LOGO_SVG: &str = "res/logo.svg";

    // 设置文件变更监控
    println!("cargo:rerun-if-changed={LOGO_RC}");
    println!("cargo:rerun-if-changed={LOGO_SVG}");

    // 检查目标平台是否需要图标
    if !need_icon() {
        return Ok(());
    }
    // 获取输出目录
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR环境变量未设置"));

    // 输出文件路径
    let logo_ico_path = out_dir.join("logo.ico");
    let svg_hash_path = out_dir.join("logo.svg.sha256");

    // 计算当前文件哈希、读取上次保存的哈希值、更新哈希文件（如果需要重新生成）
    // SVG文件变化或首次运行：生成ICO
    let current_svg_hash = file_hash(LOGO_SVG)?;
    let last_svg_hash = fs::read_to_string(&svg_hash_path).ok();
    let should_generate_ico = last_svg_hash
        .as_ref()
        .is_some_and(|hash| hash != &current_svg_hash);
    if should_generate_ico {
        fs::write(&svg_hash_path, &current_svg_hash)?;
    }
    if should_generate_ico {
        if let Err(e) = generate_ico(LOGO_SVG, &logo_ico_path) {
            eprintln!("生成ICO失败: {e}");
            return Err(e);
        }
        println!("cargo:warning=重新生成ICO文件: {logo_ico_path:?}");
    }

    // 编译资源文件
    embed_resource::compile(LOGO_RC, embed_resource::NONE)
        .manifest_optional()
        .unwrap();
    Ok(())
}
/// 该文档为AI生成。
///
/// 主入口函数
///
/// 调用资源处理函数
fn main() {
    handle_resource().expect("资源文件处理失败。");
    slint_build::compile("ui/main-window.slint").expect("Slint build failed");
}
