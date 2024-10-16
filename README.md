<h1>Typst <em>translated.</em></h1>

这里是 typstdocsinchinese 非官方中文文档项目的翻译仓库。

考虑到 Typst 文档与项目本身的高度关联性，这里保留了原 Typst 仓库的全部内容，并在此基础上进行各种更改。

## 翻译流程

WIP.

## 合并 upstream 流程

当 Typst 上游发生更新时，
- 如果更新内容与现有修改内容不冲突，则使用 GitHub 自带的 fetch upstream 功能，不定期进行同步。
- 如果更新内容与现有内容存在冲突，则利用本地 git 和 IDE 进行手动 merge **以及** 对应的新内容的翻译更新，然后以一个 merge commit 的形式提交到本仓库。（之后会视情况编写具体做法的文档 ——2024.10.16）

## 自动化流程

> [!NOTE]
> 参见
> - 本项目的流程 - [rust.yml](https://github.com/typstdocsinchinese/typst/blob/main/.github/workflows/rust.yml)
> - 前端的流程 - [webpack.yml](https://github.com/typstdocsinchinese/typstdocsinchinese.github.io/blob/main/.github/workflows/webpack.yml)

当接收到 push 时，将会构建最新的 `typst-docs` package 并用其生成文档的数据，包括 assets 和 JSON 数据文件，随后将其上传到 [typstdocsinchinese.github.io](https://github.com/typstdocsinchinese/typstdocsinchinese.github.io) 项目（以下简称前端）的相关位置。

当前端 repo 接收到 push 时，会根据现有的数据和代码执行 `npm run generate` 构建静态文件，并将生成的站点内容更新到 public branch 里，也就是 GitHub Pages 所 serve 的 branch。随后网页上的内容就会更新。

整个流程大致耗时五分钟左右。因此，当你在本项目中进行了翻译上的修改后，静候五分钟即可在网站上看到更新后的内容。偶尔会存在缓存，可用 Ctrl+Shift+R（chrome）硬性刷新。


