+ v0.1.0
    + [] 添加注释
    + [] 添加文档
    + [] 添加测试
+ v0.0.11
    + [] 简化代码
    + [] 优化加载速度
+ v0.0.10
    + [] 完善元数据的显示
+ v0.0.9
    + [] 支持放大镜(或许不需要)
    + [] 支持 orz(或许不需要)
+ v0.0.8
    + [x] 完善 cli
    + [x] 修复 bug
+ v0.0.7
    + [x] 预设大小
    + [x] 支持 zstd
+ v0.0.6
    + [x] 归位窗口
    + 解析元数据 metadata.rmg
        + [x] 显示元数据(gui)
        + [x] 显示元数据(cli)
        + [x] 导出为 json
        + [x] 由 json 导入
+ v0.0.5
    + [x] 重命名文件
```text
pad = 3

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
    + 读取配置
        + [x] 默认窗口大小
        + [x] 按键配置(或许不需要)
        + [x] 字体文件路径
    + 移动图片
        + [] 左移图片(或许不需要)
        + [] 右移图片(或许不需要)
        + [] 翻转图片(或许不需要)
    + [x] 支持 rgba
+ v0.0.4
    + [x] 解析目录下的文件
        + [x] 提取 rmg
        + [x] 排除其他文件
    + [x] 显示文字
    + [x] 浏览目录
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
