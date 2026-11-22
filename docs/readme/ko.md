# ☕ ADHD Coffee Time

[English](../../README.md) · [日本語](ja.md) · [简体中文](zh-CN.md) · [한국어](ko.md) · [Español](es.md) · [Français](fr.md) · [Deutsch](de.md) · [Português (Brasil)](pt-BR.md) · [Русский](ru.md) · [العربية](ar.md)

**타이머 켜는 걸 잊는 사람들을 위한 타이머.**

컴퓨터를 쓰면 자동으로 시작하고, 자리를 비우면 멈추고, 쉴 때가 되면 알려줍니다.

저는 성격이 급한 편입니다. 실물 뽀모도로 타이머를 샀지만 매번 직접 시작하는 과정이 번거로웠습니다. 그래서 저절로 시작하는 타이머를 만들었습니다.

기본 설정에서는 입력이 90초 동안 없으면 일시 정지하고, 10분 동안 자리를 비우면 초기화합니다. 시간은 5·10·15·25·30·60·90분 중에서 고릅니다. 테마 7종, 투명도, 선택형 알림음, S/M/L 크기(40%/70%/100%)를 설정할 수 있습니다. 설정은 재시작 후에도 유지됩니다. 10개 언어를 지원하며 시스템 트레이나 메뉴 막대에 상주합니다. 로그인 시 자동 실행도 설정할 수 있습니다.

## v0.2.0 새 기능: 집중 시간 기록과 선택형 동기화

타이머가 실제로 작동한 시간만 자동 기록하며 일시 정지나 자리 비움은 제외합니다. 타이머에는 최근 3일, 팝업에는 15분 단위로 7일의 기록을 표시합니다. 원한다면 [Google 로그인](https://adhd-coffee-focus.vercel.app/)으로 PC를 연결하고 웹에서 7일 또는 30일 기록을 확인할 수 있습니다. 계정 없이 오프라인으로 사용할 수 있으며, PC를 직접 연결했을 때만 집계된 시간 데이터를 전송합니다.

## 다운로드 및 설치

[Windows x64](https://github.com/sourcekatsudon/ADHDCoffeeTimer/releases/latest/download/ADHD-Coffee-Time_0.2.0_windows-x64-setup.exe) · [macOS Apple Silicon](https://github.com/sourcekatsudon/ADHDCoffeeTimer/releases/latest/download/ADHD-Coffee-Time_0.2.0_macos-arm64.dmg) · [macOS Intel](https://github.com/sourcekatsudon/ADHDCoffeeTimer/releases/latest/download/ADHD-Coffee-Time_0.2.0_macos-x64.dmg)

Windows에서는 exe 설치 파일을 실행하세요. Mac에서는 칩에 맞는 DMG를 열고 앱을 Applications로 드래그하세요. Mac 빌드는 미서명 또는 임시 서명 상태이며 Apple 공증을 받지 않았습니다. 네이티브 Mac UI 수동 검증도 아직 진행하지 않았습니다. Linux는 현재 사용할 수 없습니다. [플랫폼 현황](../PLATFORMS.md#linux)을 확인하세요.

기본적으로 계정이나 원격 측정 없이 로컬에서 작동하며, 클라우드 동기화는 선택 사항입니다. 마지막 OS 입력 이후의 시간만 확인하며 입력 내용은 기록하지 않습니다. 의료 또는 치료 효과를 주장하지 않는 개인용 휴식 알림 도구입니다.

[전체 설명(영어)](../../README.md) · [소스 빌드](../BUILDING.md) · [MIT 라이선스](../../LICENSE)
