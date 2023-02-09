# -*- coding: utf-8 -*-
# @Time    : 2023/2/9 18:50
# @Author  : LTstrange

import numpy as np
from synth_data import Wave, Range


class Envelope:
    def __call__(self, wave: Wave, time_range: Range, sample_rate: float):
        raise NotImplementedError


class ADSR(Envelope):
    def __init__(self, attack: float, decay: float, sustain_level: float, release: float):
        """
        fade in/out envelope
        :param attack: The time for the amplitude to increase from 0 to 1.
        :param decay: The time for the amplitude to decrease from 1 to sustain.
        :param sustain_level: A fraction of the maximum amplitude (0 to 1).
        :param release: The time for the amplitude to decrease from sustain to 0.
        """
        self.attack = attack  # second
        self.decay = decay  # second
        self.sustain_level = sustain_level  # fraction
        self.release = release  # second

    def __call__(self, wave: Wave, time_range: Range, sample_rate: int):
        # Calculate the length of each stage in samples
        attack_samples = int(self.attack * sample_rate)
        decay_samples = int(self.decay * sample_rate)
        sustain = time_range.duration - self.attack - self.decay - self.release
        sustain_samples = int(sustain * sample_rate) if sustain > 0 else 0
        release_samples = int(self.release * sample_rate)

        sample_length = attack_samples + decay_samples + sustain_samples + release_samples

        # Initialize the envelope
        envelope = np.zeros(sample_length)

        # Attack stage
        envelope[:attack_samples] = np.linspace(0, 1, attack_samples)
        # Decay stage
        envelope[attack_samples:attack_samples + decay_samples] = np.linspace(1, self.sustain_level, decay_samples)
        # Sustain stage
        envelope[attack_samples + decay_samples:attack_samples + decay_samples + sustain_samples] = self.sustain_level
        # Release stage
        envelope[-release_samples:] = np.linspace(self.sustain_level, 0, release_samples)

        wave_samples = wave.render(time_range.duration, sample_length)
        return wave_samples * envelope