"""Generate gentle notification auditions; does not modify the app's sound."""

from pathlib import Path
import array
import math
import wave

RATE = 44100
ROOT = Path(__file__).resolve().parent.parent / "src-tauri" / "sounds"


def tone(freq, duration, attack, decay, harmonics=(1.0, 0.06, 0.012)):
    result = []
    for i in range(round(duration * RATE)):
        t = i / RATE
        # Zero slope at onset and end avoids clicks and a sharp initial strike.
        onset = math.sin(min(t / attack, 1.0) * math.pi / 2) ** 2
        release = math.sin(min((duration - t) / 0.45, 1.0) * math.pi / 2) ** 2
        envelope = onset * math.exp(-t / decay) * release
        carrier = sum(a * math.sin(2 * math.pi * freq * (n + 1) * t)
                      for n, a in enumerate(harmonics))
        result.append(envelope * carrier)
    return result


def compose(events, duration):
    samples = [0.0] * round(duration * RATE)
    for start, freq, length, attack, decay, gain, harmonics in events:
        offset = round(start * RATE)
        for i, value in enumerate(tone(freq, length, attack, decay, harmonics)):
            if offset + i < len(samples):
                samples[offset + i] += 0.75 * gain * value
    return samples


def write(name, samples):
    assert max(abs(x) for x in samples) < 0.20
    pcm = array.array('h', (round(x * 32767) for x in samples))
    import sys
    if sys.byteorder != 'little':
        pcm.byteswap()
    with wave.open(str(ROOT / name), 'wb') as w:
        w.setparams((1, 2, RATE, 0, 'NONE', 'not compressed'))
        w.writeframes(pcm.tobytes())


pure = (1.0, 0.012)
samples = compose([(0.12, 880.0, 2.8, 0.12, 0.85, 0.16, pure)], 3.1)
write("seatbelt.wav", samples)
print("Generated the original single-note notification sound.")
