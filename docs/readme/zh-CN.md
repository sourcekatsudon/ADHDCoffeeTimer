# ☕ ADHD Coffee Time

[English](../../README.md) · [日本語](ja.md) · [简体中文](zh-CN.md) · [한국어](ko.md) · [Español](es.md) · [Français](fr.md) · [Deutsch](de.md) · [Português (Brasil)](pt-BR.md) · [Русский](ru.md) · [العربية](ar.md)

**为总是忘记开计时器的人设计的计时器。**

开始用电脑，它就开始计时；离开座位，它会暂停；该休息时，它会提醒你。

我性子比较急，虽然买了一个实体番茄钟，但每次都要手动启动实在太麻烦。于是，我做了一个会自己开始计时的工具。

默认情况下，90秒没有输入就暂停，离开10分钟后重置。可选时长为5、10、15、25、30、60或90分钟。提供7种主题、透明度调节、可选提示音，以及S／M／L三种尺寸（40%／70%／100%）。设置会保存在本机，重启后仍然有效。界面支持10种语言，应用可常驻系统托盘或菜单栏。

## 新功能：专注时间记录与可选同步

v0.2.0 会自动记录计时器实际运行时间（暂停或离开时不计入）。计时器显示最近 3 天的记录，弹窗显示 7 天的 15 分钟时间格。你可以选择使用 [Google 账号同步](https://adhd-coffee-focus.vercel.app/) 多台电脑，并在网页上查看 7 天或 30 天的汇总。无需账号也可离线使用；仅在主动配对设备后上传汇总时间记录，不采集键盘输入内容。

## 下载与安装

[Windows x64](https://github.com/sourcekatsudon/ADHDCoffeeTimer/releases/latest/download/ADHD-Coffee-Time_0.2.0_windows-x64-setup.exe) · [macOS Apple Silicon](https://github.com/sourcekatsudon/ADHDCoffeeTimer/releases/latest/download/ADHD-Coffee-Time_0.2.0_macos-arm64.dmg) · [macOS Intel](https://github.com/sourcekatsudon/ADHDCoffeeTimer/releases/latest/download/ADHD-Coffee-Time_0.2.0_macos-x64.dmg)

Windows：运行exe安装程序。Mac：选择对应芯片的DMG，打开后将应用拖到Applications。Mac版本未签名或仅采用临时签名，未经Apple公证，原生Mac界面尚未经过人工验证。Linux目前不可用，详见[平台状态](../PLATFORMS.md#linux)。

默认无需账号即可在本机使用，也不进行遥测。只有在主动配对电脑后才会将汇总的专注时间记录同步到云端。应用仅读取距上次系统输入的时间，不记录你输入的内容。这是休息提醒工具，不作医疗或治疗承诺。

[完整说明（英语）](../../README.md) · [从源码构建](../BUILDING.md) · [MIT许可证](../../LICENSE)
