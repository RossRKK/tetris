#!/usr/bin/env python3
"""
GBS File Analyzer - Extracts note frequencies, timings, and rhythms from GameBoy Sound files
Specifically designed to analyze tetris.gbs and output timing/frequency data
"""

import struct
import sys
from typing import Dict, List, Tuple, Optional

from math import floor

class GBSAnalyzer:
    # GameBoy sound channel frequencies
    CHANNEL_FREQUENCIES = {
        0x00: 0,    # Rest/silence
        0x01: 65.4, 0x02: 69.3, 0x03: 73.4, 0x04: 77.8, 0x05: 82.4,
        0x06: 87.3, 0x07: 92.5, 0x08: 98.0, 0x09: 103.8, 0x0A: 110.0,
        0x0B: 116.5, 0x0C: 123.5, 0x0D: 130.8, 0x0E: 138.6, 0x0F: 146.8,
        0x10: 155.6, 0x11: 164.8, 0x12: 174.6, 0x13: 185.0, 0x14: 196.0,
        0x15: 207.7, 0x16: 220.0, 0x17: 233.1, 0x18: 246.9, 0x19: 261.6,
        0x1A: 277.2, 0x1B: 293.7, 0x1C: 311.1, 0x1D: 329.6, 0x1E: 349.2,
        0x1F: 370.0, 0x20: 392.0, 0x21: 415.3, 0x22: 440.0, 0x23: 466.2,
        0x24: 493.9, 0x25: 523.3, 0x26: 554.4, 0x27: 587.3, 0x28: 622.3,
        0x29: 659.3, 0x2A: 698.5, 0x2B: 740.0, 0x2C: 784.0, 0x2D: 830.6,
        0x2E: 880.0, 0x2F: 932.3, 0x30: 987.8, 0x31: 1046.5, 0x32: 1108.7,
        0x33: 1174.7, 0x34: 1244.5, 0x35: 1318.5, 0x36: 1396.9, 0x37: 1480.0
    }
    
    NOTE_NAMES = {
        65.4: "C2", 69.3: "C#2", 73.4: "D2", 77.8: "D#2", 82.4: "E2",
        87.3: "F2", 92.5: "F#2", 98.0: "G2", 103.8: "G#2", 110.0: "A2",
        116.5: "A#2", 123.5: "B2", 130.8: "C3", 138.6: "C#3", 146.8: "D3",
        155.6: "D#3", 164.8: "E3", 174.6: "F3", 185.0: "F#3", 196.0: "G3",
        207.7: "G#3", 220.0: "A3", 233.1: "A#3", 246.9: "B3", 261.6: "C4",
        277.2: "C#4", 293.7: "D4", 311.1: "D#4", 329.6: "E4", 349.2: "F4",
        370.0: "F#4", 392.0: "G4", 415.3: "G#4", 440.0: "A4", 466.2: "A#4",
        493.9: "B4", 523.3: "C5", 554.4: "C#5", 587.3: "D5", 622.3: "D#5",
        659.3: "E5", 698.5: "F5", 740.0: "F#5", 784.0: "G5", 830.6: "G#5",
        880.0: "A5", 932.3: "A#5", 987.8: "B5", 1046.5: "C6", 1108.7: "C#6",
        1174.7: "D6", 1244.5: "D#6", 1318.5: "E6", 1396.9: "F6", 1480.0: "F#6"
    }

    def __init__(self, filename: str):
        self.filename = filename
        self.header = {}
        self.music_data = b''
        
    def parse_header(self, data: bytes) -> Dict:
        """Parse GBS header according to specification"""
        if len(data) < 0x70:
            raise ValueError("File too small to contain valid GBS header")
            
        # Check identifier
        identifier = data[0:3].decode('ascii', errors='ignore')
        if identifier != 'GBS':
            raise ValueError(f"Invalid GBS file - identifier is '{identifier}', expected 'GBS'")
        
        # Parse header fields
        version = data[3]
        num_songs = data[4]
        first_song = data[5]
        load_addr = struct.unpack('<H', data[6:8])[0]  # Little endian
        init_addr = struct.unpack('<H', data[8:10])[0]
        play_addr = struct.unpack('<H', data[10:12])[0]
        stack_ptr = struct.unpack('<H', data[12:14])[0]
        timer_modulo = data[14]
        timer_control = data[15]
        
        # Extract strings (null-terminated or 32 chars max)
        title = data[16:48].split(b'\0')[0].decode('ascii', errors='ignore')
        author = data[48:80].split(b'\0')[0].decode('ascii', errors='ignore')
        copyright = data[80:112].split(b'\0')[0].decode('ascii', errors='ignore')
        
        return {
            'identifier': identifier,
            'version': version,
            'num_songs': num_songs,
            'first_song': first_song,
            'load_addr': load_addr,
            'init_addr': init_addr,
            'play_addr': play_addr,
            'stack_ptr': stack_ptr,
            'timer_modulo': timer_modulo,
            'timer_control': timer_control,
            'title': title,
            'author': author,
            'copyright': copyright
        }
    
    def calculate_timing_rate(self) -> float:
        """Calculate playback rate from timer settings"""
        tac = self.header['timer_control']
        tma = self.header['timer_modulo']
        
        if tac & 0x04 == 0:  # Use v-blank (bit 2 = 0)
            return 59.7  # Hz
        else:
            # Timer interrupt rates
            counter_rates = {
                0: 4096,    # 00
                1: 262144,  # 01  
                2: 65536,   # 10
                3: 16384    # 11
            }
            rate_bits = tac & 0x03
            counter_rate = counter_rates[rate_bits]
            
            # Apply 2x CPU clock if bit 7 is set
            if tac & 0x80:
                counter_rate *= 2
                
            return counter_rate / (256 - tma)
    
    def analyze_music_data(self, data: bytes, start_offset: int = 0x70) -> Dict[int, List[Dict]]:
        """Analyze the music data section for note patterns, separated by songs"""
        all_songs = {}
        
        # Analyze each song separately
        for song_num in range(1, self.header['num_songs'] + 1):
            notes = []
            offset = start_offset
            timestamp = 0.0
            playback_rate = self.calculate_timing_rate()
            time_per_frame = 1.0 / playback_rate
            
            # For each song, look for patterns in different sections of the data
            # This is a simplified approach - real GBS parsing would need to simulate the Z80 code
            song_data_start = start_offset + (song_num - 1) * 1024  # Estimate song sections
            song_data_end = min(song_data_start + 1024, len(data))
            
            offset = song_data_start
            while offset < song_data_end - 1:
                byte = data[offset]
                
                # Look for potential sound register writes
                if byte in self.CHANNEL_FREQUENCIES:
                    freq = self.CHANNEL_FREQUENCIES[byte]
                    if freq > 0:  # Skip silence/rest
                        note_name = self.NOTE_NAMES.get(freq, f"{freq:.1f}Hz")
                        
                        # Try to determine channel (channels 1-4 on GameBoy)
                        channel = self._guess_channel(offset, data)
                        
                        # Estimate duration by looking ahead for pattern changes
                        duration = self._estimate_duration(offset, data, time_per_frame)
                        
                        notes.append({
                            'timestamp': timestamp,
                            'frequency': freq,
                            'note': note_name,
                            'channel': channel,
                            'duration': duration,
                            'offset': hex(offset),
                            'song': song_num
                        })
                        
                        timestamp += duration
                
                offset += 1
            
            if notes:  # Only add songs that have notes
                all_songs[song_num] = notes
                
        return all_songs
    
    def _guess_channel(self, offset: int, data: bytes) -> int:
        """Heuristic to guess which GameBoy sound channel based on context"""
        # Look at surrounding bytes for clues about sound register usage
        context = data[max(0, offset-2):offset+3]
        
        # GameBoy sound channels have different typical frequency ranges
        byte = data[offset]
        freq = self.CHANNEL_FREQUENCIES.get(byte, 0)
        
        if freq < 200:
            return 1  # Channel 1 (square wave) - lower frequencies
        elif freq < 500:
            return 2  # Channel 2 (square wave) - mid frequencies  
        elif freq < 1000:
            return 3  # Channel 3 (wave) - higher frequencies
        else:
            return 4  # Channel 4 (noise) - very high frequencies
    
    def _estimate_duration(self, offset: int, data: bytes, base_duration: float) -> float:
        """Estimate note duration by looking for pattern repetition"""
        # Simple heuristic: look ahead for similar patterns
        current_byte = data[offset]
        duration_multiplier = 1
        
        # Look ahead to find when the pattern changes significantly
        for i in range(1, min(16, len(data) - offset)):
            if data[offset + i] == current_byte:
                duration_multiplier += 1
            else:
                break
                
        return base_duration * max(1, duration_multiplier // 4)
    
    def load_and_parse(self):
        """Load and parse the GBS file"""
        try:
            with open(self.filename, 'rb') as f:
                data = f.read()
        except FileNotFoundError:
            raise FileNotFoundError(f"GBS file '{self.filename}' not found")
        
        self.header = self.parse_header(data)
        self.music_data = data[0x70:]  # Music data starts after header
        
        return self.analyze_music_data(data)
    
    def export_analysis(self, songs_data: Dict[int, List[Dict]], output_file: str):
        """Export analysis results to a file"""
        with open(output_file, 'w') as f:
            f.write("Tetris GBS File Analysis\n")
            f.write("=" * 50 + "\n\n")
            
            # Header information
            f.write("HEADER INFORMATION:\n")
            f.write(f"Title: {self.header['title']}\n")
            f.write(f"Author: {self.header['author']}\n") 
            f.write(f"Copyright: {self.header['copyright']}\n")
            f.write(f"Number of songs: {self.header['num_songs']}\n")
            f.write(f"Playback rate: {self.calculate_timing_rate():.1f} Hz\n")
            f.write(f"Load address: ${self.header['load_addr']:04X}\n")
            f.write(f"Init address: ${self.header['init_addr']:04X}\n")
            f.write(f"Play address: ${self.header['play_addr']:04X}\n\n")
            
            # Song analysis
            total_notes = 0
            song_file = open('tetris_songs.rs', 'w')
            song_file.write("use crate::synthesizer::Note;\n")
            for song_num, notes in songs_data.items():
                f.write(f"SONG {song_num} ANALYSIS:\n")
                f.write("Time(s)\tFreq(Hz)\tNote\tChannel\tDuration(s)\tOffset\n")
                f.write("-" * 60 + "\n")
                
                for note in notes:
                    f.write(f"{note['timestamp']:.3f}\t{note['frequency']:.1f}\t"
                           f"{note['note']}\t{note['channel']}\t{note['duration']:.3f}\t"
                           f"{note['offset']}\n")
                
                f.write(f"\nSong {song_num} notes: {len(notes)}\n")
                f.write("=" * 50 + "\n\n")
                total_notes += len(notes)

                song_file.write(f"""pub const SONG_{song_num}: [Note; {len(notes)}] = [\n""")
                for note in notes:
                    song_file.write(f"""    Note {{start_time: {floor(note['timestamp'] * 44100)}, end_time: {floor((note['timestamp'] + note['duration']) * 44100)}, volume: {1.0}, interval_length:{floor(44100/note['frequency'])},}},\n""")
                song_file.write("""];""")

            
            f.write(f"Total notes analyzed across all songs: {total_notes}\n")

def main():
    if len(sys.argv) != 2:
        print("Usage: python3 gbs_analyzer.py <gbs_file>")
        sys.exit(1)
        
    gbs_file = sys.argv[1]
    analyzer = GBSAnalyzer(gbs_file)
    
    try:
        print(f"Analyzing {gbs_file}...")
        notes = analyzer.load_and_parse()
        
        output_file = gbs_file.replace('.gbs', '_analysis.txt')
        analyzer.export_analysis(notes, output_file)
        
        print(f"Analysis complete! Results saved to {output_file}")
        print(f"Found {len(notes)} note events")
        
    except Exception as e:
        print(f"Error analyzing GBS file: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()