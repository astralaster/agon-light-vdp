pub mod audio {
    use sdl2::AudioSubsystem;
    use std::sync::mpsc::{Sender, Receiver};
    use std::sync::mpsc;
    use sdl2::audio::{AudioCallback, AudioSpecDesired, AudioSpec, AudioDevice};

    // Parameters of a generated tone.
    #[derive(Clone)]
    struct WaveformState {
        period: f32,
        phase: f32,
        volume: f32,
        samples_to_go: i32,
    }

    #[derive(Debug)]
    struct GeneratorMessage {
        channel: u8,
        waveform: u8,
        volume: u8,
        frequency: i16,
        duration: i16,
    }

    
    struct Generator {
        generators: Vec<WaveformState>,
        rx_fromVDP: Receiver<GeneratorMessage>,
        tx_toVDP: Sender<u8>,
        audio_spec: AudioSpec,
    }

    impl AudioCallback for Generator {
        type Channel = f32;

        fn callback(&mut self, out: &mut [f32]) {
            while true {
                match self.rx_fromVDP.try_recv() {
                    Ok(msg) => {
                        log::info!("Message received: {:?} freq {} buflen{}",msg,self.audio_spec.freq,out.len());
                        let gen = &mut self.generators[msg.channel as usize];
                        gen.period=(self.audio_spec.freq as f32)/(msg.frequency as f32);
                        gen.phase = 0.0;
                        gen.volume = (msg.volume as f32)* (0.6/255.0);
                        gen.samples_to_go = ((msg.duration as i32)*(self.audio_spec.freq as i32))/1000  as i32;
                    },
                    Err(_) => {break;},
                }
            }
            for x in out.iter_mut() {
                let mut sample = 0.0;
                let mut cn=0;
                for gen in self.generators.iter_mut() {
                    if gen.samples_to_go > 0 {
                        sample += (gen.phase - gen.period/2.0)/gen.period*gen.volume;
                        gen.phase = gen.phase+1.0;
                        if (gen.phase >= gen.period) {
                            gen.phase -= gen.period;
                        }          
                        gen.samples_to_go-=1;
                        if gen.samples_to_go==0 {
                            self.tx_toVDP.send(cn as u8);
                        }
                    }
                    cn += 1;
                }
                *x = sample;
            }
        }
    }

    
    pub struct AudioChannels {
        nchannels: i32,
        channels_busy: Vec<bool>,
        tx_to_audio: Sender<GeneratorMessage>,
        rx_from_audio: Receiver<u8>,
        device: AudioDevice<Generator>,
    }

    impl AudioChannels {
        
        pub fn new(audio_subsystem: sdl2::AudioSubsystem) -> AudioChannels {
            let (tx_VDP2audio, rx_VDP2audio): (Sender<GeneratorMessage>, Receiver<GeneratorMessage>) = mpsc::channel();
            let (tx_audio2VDP, rx_audio2VDP): (Sender<u8>, Receiver<u8>) = mpsc::channel();
            let desired_spec = AudioSpecDesired {
                freq: Some(44100),
                channels: Some(1),
                samples: None,
            };
            let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            Generator {
                generators: vec![WaveformState{period: 0.0, phase: 0.0, volume: 0.0, samples_to_go: 0 };3],
                rx_fromVDP: rx_VDP2audio,
                tx_toVDP: tx_audio2VDP,
                audio_spec: spec
            }}).unwrap();
            device.resume();
            AudioChannels{nchannels: 3,
                          channels_busy: vec![false, false, false],
                          tx_to_audio: tx_VDP2audio,
                          rx_from_audio: rx_audio2VDP,
                          device: device,
            }
        }

        pub fn start_tone(&mut self, channel: u8, waveform: u8, volume: u8,
                          frequency: i16, duration: i16) -> bool {
            while true {
                match self.rx_from_audio.try_recv() {
                    Ok(b) => {
                        if (b as i32)  < self.nchannels {
                            // Clear the busy state if a generator reports finished.
                            self.channels_busy[b as usize] = false;
                        }
                    },
                    Err(_) => {break;}
                }
            }
            if ((channel as i32) >= self.nchannels || self.channels_busy[channel as usize]) {
                false
            } else {
                log::info!("Trying to play note on chan {} vol {} freq {} duration {}",channel,volume,frequency,duration);
                if duration > 0 {
                    self.tx_to_audio.send(GeneratorMessage {
                        channel: channel,
                        waveform: waveform,
                        volume: volume,
                        frequency: frequency,
                        duration: duration});
                    self.channels_busy[channel as usize] = true;
                }
                true
            }
        }
    }
    
}
