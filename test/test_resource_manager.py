from unittest.mock import call, patch

import numpy as np
import pytest

from plaintext_daw.models.instrument import Instrument
from plaintext_daw.models.pattern import Pattern

from plaintext_daw.resource_manager import ResourceManager


class TestResourceManager:

    def setup_method(self, test_method):
        self.rm = ResourceManager('.')

    def test_working_dir(self):
        assert self.rm.working_dir == '.'

    def test_check_types(self):
        config = {
            'foo': 'val1',
            'bar': 'val2',
        }
        ResourceManager.check_types(config, ['foo', 'bar'])
        with pytest.raises(ValueError, match='baz'):
            ResourceManager.check_types(config, ['foo', 'bar', 'baz'])

    @patch.object(ResourceManager, 'get_clip')
    @patch.object(ResourceManager, 'get_instrument')
    @patch.object(ResourceManager, 'get_pattern')
    def test_get_song(self, mock_get_pattern, mock_get_instrument,mock_get_clip):
        with pytest.raises(ValueError):
            self.rm.get_song({'bpm': 100})
        
        clip_dict = {
            'costco': {'pancake': 'wait'},
            'walmart': {'no': 'free samples'},
        }
        ins_dict = {
            'myins': {'field': 'hello'},
            'yourins': {'name': 'insurance??'},
        }
        pattern_dict = {
            'pattern': {'hey': 'there'},
            'other': {'angle': 'momentum'},
        }
        song = self.rm.get_song({
            'bpm': 100,
            'sample_rate': 44100,
            'clips': clip_dict,
            'instruments': ins_dict,
            'patterns': pattern_dict,
        })
        assert song.bpm == 100
        assert song.sample_rate == 44100
        mock_get_clip.assert_has_calls([
            call(clip_dict['costco']),
            call(clip_dict['walmart']),
        ])
        mock_get_instrument.assert_has_calls([
            call(ins_dict['myins']),
            call(ins_dict['yourins']),
        ])
        mock_get_pattern.assert_has_calls([
            call(pattern_dict['pattern']),
            call(pattern_dict['other']),
        ])


    def test_get_clip_no_type(self):
        with pytest.raises(ValueError):
            self.rm.get_clip({})
        with pytest.raises(ValueError):
            self.rm.get_clip({'type': 'something crazy'})

    def test_get_clip_wav(self):
        # Good config
        wav_path = 'test/data/song1/piano/Piano-A0.ogg.wav'
        config = {
            'type': 'wav',
            'path': wav_path,
        }
        clip = self.rm.get_clip(config)
        assert isinstance(clip.data, np.ndarray)
        assert clip.channels == 1
        assert clip.sample_width == 2
        assert clip.sample_rate == 44100

        # Missing fields

    def test_get_clip_synth(self):
        # Good config
        config = {
            'type': 'synth',
            'frequency': 16.35,
            'sample_rate': 5000,
            'length': 2,
        }
        clip = self.rm.get_clip(config)
        assert isinstance(clip.data, np.ndarray)
        assert len(clip.data) == 5000 * 2 # sample_rate * length
        assert clip.channels == 1
        assert clip.sample_width == 2
        assert clip.sample_rate == 5000

        # Missing fields
        for missing_field in ['frequency', 'sample_rate', 'length']:
            bad_cfg = config.copy()
            del bad_cfg[missing_field]
            with pytest.raises(ValueError):
                self.rm.get_clip(bad_cfg)

    @patch.object(ResourceManager, 'get_clip')
    def test_get_instrument(self, mock_get_clip):
        clips_dict = {
            'A0': {'path': 'path/to/A0.wav'},
            'C5': {'path': 'path/to/C5.wav'},
        }
        instrument = self.rm.get_instrument({
            'clips': clips_dict,
        })
        assert isinstance(instrument, Instrument)
        mock_get_clip.assert_has_calls([
            call(clips_dict['A0']),
            call(clips_dict['C5']),
        ])

    def test_get_instrument_git(self):
        pass # TODO

    @patch.object(ResourceManager, 'get_note')
    def test_get_pattern(self, mock_get_note):
        notes = [
            {'value': 'A0', 'start': 0, 'length': 1},
            {'value': 'A1', 'start': 1, 'length': 2},
        ]
        pattern = self.rm.get_pattern({
            'instrument': 'piano',
            'start': 12,
            'repeat': 0,
            'notes': notes,
        })
        assert isinstance(pattern, Pattern)
        assert pattern.start == 12
        assert pattern.repeat == 0
        mock_get_note.assert_has_calls([call(n) for n in notes])

    def test_get_note(self):
        pass
