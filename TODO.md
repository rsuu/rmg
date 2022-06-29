+ v0.1.2
    + [] 添加注释
    + [] 添加文档
    + [] 添加测试
+ v0.1.1
    + [] 修复 bug
+ v0.1.0
    + [] 简化代码
    + [] 优化加载速度
+ v0.0.9
    + [] 支持放大镜(或许不需要)
+ v0.0.8
    + [] 完善 cli
+ v0.0.7
   + [] 支持 zstd && orz
+ v0.0.6
    + [] 归位窗口
+ v0.0.5
    + [] 重命名文件
```text
cbb comic/ --pad=3 --dry-run
1.png    -> 001.png
8.png    -> 008.png
3.png    -> 003.png
4.png    -> 004.png
6.png    -> 006.png
2.png    -> 002.png
9.png    -> 009.png
5.png    -> 005.png
11.png   -> 011.png
7.png    -> 007.png
10.png   -> 010.png
```
    + [] 读取 toml 配置
        + [] 默认窗口大小
        + [] 按键配置(或许不需要)
    + [] 移动图片
        + [] 左移图片
        + [] 右移图片
        + [] 翻转图片
+ v0.0.4
    + [] 解析元数据 metadata.rmg
        + [] 显示元数据(gui)
        + [] 显示元数据(cli)
        + [] 导出为 json
        + [] 由 json 导入
    + [x] 解析目录下的文件
        + [] 提取 rmg
        + [x] 排除其他文件
    + [] 显示文字
+ v0.0.3
    + [x] 修复 bug
+ v0.0.2
    + [x] 自动设定图片大小
    + [x] 惰性加载图片(提取部分文件)
+ v0.0.1
    + [x] 滚动图片


+ lib
    + https://github.com/Cykooz/fast_image_resize
    + https://github.com/rust-lang/portable-simd
    + https://github.com/Uskrai/fmr-rs
    + https://stackoverflow.com/questions/67823680/open-a-single-file-from-a-zip-archive-and-pass-on-as-read-instance
    + https://stackoverflow.com/questions/61604736/reading-zip-file-in-rust-causes-data-owned-by-the-current-function
    + https://github.com/alexcrichton/tar-rs/blob/master/examples/extract_file.rs
    + https://stackoverflow.com/questions/69966292/how-decompress-and-unpack-tar-gz-archive-in-download-process
    + https://docs.rs/zip/latest/zip/read/struct.ZipArchive.html
    + https://github.com/gyscos/zstd-rs
    + https://github.com/Soft/xcolor
    + https://github.com/richox/orz

+ idea
    + https://lib.rs/crates/comic-book-binder

+ format
    + https://en.wikipedia.org/wiki/Comic_book_archive
