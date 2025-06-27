import mido
import sys

SAMPLE_RATE = 44100

NOTE_FREQS = {
    n: 440.0 * 2 ** ((n - 69) / 12.0) for n in range(128)
}

def freq_to_interval(freq):
    return int(SAMPLE_RATE / freq)

def midi_to_rust_notes(midi_file):
    mid = mido.MidiFile(midi_file)
    tempo = 500000  # default tempo: 120bpm TODO: should be 150
    ticks_per_beat = mid.ticks_per_beat

    events = []
    time = 0
    for track in mid.tracks:
        abs_time = 0
        on_notes = {}
        for msg in track:
            abs_time += msg.time
            if msg.type == 'set_tempo':
                tempo = msg.tempo
            elif msg.type == 'note_on' and msg.velocity > 0:
                on_notes[msg.note] = (abs_time, msg.velocity)
            elif (msg.type == 'note_off') or (msg.type == 'note_on' and msg.velocity == 0):
                if msg.note in on_notes:
                    start_tick, velocity = on_notes.pop(msg.note)
                    end_tick = abs_time

                    start_sec = mido.tick2second(start_tick, ticks_per_beat, tempo)
                    end_sec = mido.tick2second(end_tick, ticks_per_beat, tempo)

                    start_sample = int(start_sec * SAMPLE_RATE)
                    end_sample = int(end_sec * SAMPLE_RATE)
                    volume = velocity / 127.0
                    interval_length = freq_to_interval(NOTE_FREQS[msg.note])

                    events.append({
                        'start_time': start_sample,
                        'end_time': end_sample,
                        'volume': volume,
                        'interval_length': interval_length
                    })
    return events

def write_rust_file(notes, output_file="tetris_songs.rs", song_name="SONG_3"):
    with open(output_file, "w") as f:
        f.write("use crate::synthesizer::Note;\n\n")
        f.write(f"pub const {song_name}: [Note; {len(notes)}] = [\n")
        for note in notes:
            f.write(f"    Note {{ start_time: {note['start_time']}, end_time: {note['end_time']}, volume: {note['volume']:.3f}, interval_length: {note['interval_length']} }},\n")
        f.write("];\n")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python midi_to_rust.py <midi_file>")
        sys.exit(1)

    midi_file = sys.argv[1]
    notes = midi_to_rust_notes(midi_file)
    write_rust_file(notes)
    print(f"Generated tetris_songs.rs with {len(notes)} notes.")
