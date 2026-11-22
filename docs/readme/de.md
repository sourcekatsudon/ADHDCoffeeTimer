# ☕ ADHD Coffee Time

[English](../../README.md) · [日本語](ja.md) · [简体中文](zh-CN.md) · [한국어](ko.md) · [Español](es.md) · [Français](fr.md) · [Deutsch](de.md) · [Português (Brasil)](pt-BR.md) · [Русский](ru.md) · [العربية](ar.md)

**Ein Timer für alle, die ihren Timer vergessen.**

Er startet, wenn du den Computer benutzt, pausiert, wenn du weg bist, und erinnert dich an deine Pause.

Ich bin ungeduldig. Ich habe einen physischen Pomodoro-Timer gekauft, aber ihn jedes Mal selbst zu starten, war mir ein Schritt zu viel. Also habe ich einen gebaut, der von selbst losläuft.

Standardmäßig pausiert er nach 90 Sekunden ohne Eingabe und wird nach 10 Minuten Abwesenheit zurückgesetzt. Wähle 5, 10, 15, 25, 30, 60 oder 90 Minuten, sieben Designs, Transparenz, einen optionalen Ton und S/M/L (40 %/70 %/100 %). Einstellungen bleiben nach einem Neustart erhalten. Die App bietet zehn Sprachen und bleibt im Infobereich oder in der Menüleiste.

## Neu in v0.2.0: Fokusverlauf und optionale Synchronisierung

Die App erfasst nur die tatsächliche Laufzeit des Timers, nicht Pausen oder Abwesenheit. Die letzten 3 Tage erscheinen auf dem Timer; ein Popup zeigt 7 Tage in 15-Minuten-Feldern. Optional können Sie PCs über [Google-Anmeldung](https://adhd-coffee-focus.vercel.app/) verbinden und 7 oder 30 Tage im Web vergleichen. Ohne Konto funktioniert die App weiterhin offline; nur nach einer ausdrücklichen Geräteverknüpfung werden zusammengefasste Zeitdaten synchronisiert.

## Herunterladen und installieren

[Windows x64](https://github.com/sourcekatsudon/ADHDCoffeeTimer/releases/latest/download/ADHD-Coffee-Time_0.2.0_windows-x64-setup.exe) · [macOS Apple Silicon](https://github.com/sourcekatsudon/ADHDCoffeeTimer/releases/latest/download/ADHD-Coffee-Time_0.2.0_macos-arm64.dmg) · [macOS Intel](https://github.com/sourcekatsudon/ADHDCoffeeTimer/releases/latest/download/ADHD-Coffee-Time_0.2.0_macos-x64.dmg)

Windows: exe-Installationsdatei ausführen. Mac: das zum Prozessor passende DMG öffnen und die App in Applications ziehen. Mac-Versionen sind unsigniert oder ad hoc signiert und nicht notarisiert. Die native Mac-Oberfläche wurde noch nicht manuell geprüft. Linux ist nicht verfügbar; siehe [Plattformstatus](../PLATFORMS.md#linux).

Standardmäßig funktioniert alles lokal, ohne Konto oder Telemetrie; eine Cloud-Synchronisierung ist freiwillig. Gelesen wird die Zeit seit der letzten Systemeingabe, niemals deren Inhalt. Die App ist eine persönliche Erinnerungshilfe ohne medizinische Versprechen.

[Ausführliche Anleitung auf Englisch](../../README.md) · [Selbst kompilieren](../BUILDING.md) · [MIT-Lizenz](../../LICENSE)
