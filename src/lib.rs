use std::sync::mpsc::{channel, Receiver};

use gdnative::prelude::*;
use tts::{Features, Tts, UtteranceId};

#[derive(NativeClass)]
struct Utterance(pub(crate) Option<UtteranceId>);

#[methods]
impl Utterance {
    fn new(_owner: &Reference) -> Self {
        Self(None)
    }
}

#[allow(clippy::enum_variant_names)]
enum Msg {
    UtteranceBegin(UtteranceId),
    UtteranceEnd(UtteranceId),
    UtteranceStop(UtteranceId),
}

#[allow(clippy::upper_case_acronyms)]
#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register)]
struct TTS(Tts, Receiver<Msg>);

#[methods]
impl TTS {
    fn new(owner: &Node) -> Self {
        owner.set_pause_mode(2);
        let tts = Tts::default().expect("Failed to initialize TTS");
        let (tx, rx) = channel();
        let Features {
            utterance_callbacks,
            ..
        } = tts.supported_features();
        if utterance_callbacks {
            let tx_end = tx.clone();
            let tx_stop = tx.clone();
            tts.on_utterance_begin(Some(Box::new(move |utterance| {
                tx.send(Msg::UtteranceBegin(utterance))
                    .expect("Failed to send UtteranceBegin");
            })))
            .expect("Failed to set utterance_begin callback");
            tts.on_utterance_end(Some(Box::new(move |utterance| {
                tx_end
                    .send(Msg::UtteranceEnd(utterance))
                    .expect("Failed to send UtteranceEnd");
            })))
            .expect("Failed to set utterance_end callback");
            tts.on_utterance_stop(Some(Box::new(move |utterance| {
                tx_stop
                    .send(Msg::UtteranceStop(utterance))
                    .expect("Failed to send UtteranceStop");
            })))
            .expect("Failed to set utterance_stop callback");
        }
        Self(tts, rx)
    }

    fn register(builder: &ClassBuilder<Self>) {
        builder
            .property("volume")
            .with_getter(|this: &TTS, _| match this.0.get_volume() {
                Ok(volume) => volume,
                _ => 0.,
            })
            .with_setter(|this: &mut TTS, _, v: f32| {
                let Features {
                    volume: volume_supported,
                    ..
                } = this.0.supported_features();
                if volume_supported {
                    let mut v = v;
                    if v < this.0.min_volume() {
                        v = this.0.min_volume();
                    } else if v > this.0.max_volume() {
                        v = this.0.max_volume();
                    }
                    this.0.set_volume(v).expect("Failed to set volume");
                }
            })
            .done();
        builder
            .property("min_volume")
            .with_getter(|this: &TTS, _| {
                let Features {
                    volume: volume_supported,
                    ..
                } = this.0.supported_features();
                if volume_supported {
                    this.0.min_volume()
                } else {
                    0.
                }
            })
            .done();
        builder
            .property("max_volume")
            .with_getter(|this: &TTS, _| {
                let Features {
                    volume: volume_supported,
                    ..
                } = this.0.supported_features();
                if volume_supported {
                    this.0.max_volume()
                } else {
                    0.
                }
            })
            .done();
        builder
            .property("normal_volume")
            .with_getter(|this: &TTS, _| {
                let Features {
                    volume: volume_supported,
                    ..
                } = this.0.supported_features();
                if volume_supported {
                    this.0.normal_volume()
                } else {
                    0.
                }
            })
            .done();
        builder
            .property("rate")
            .with_getter(|this: &TTS, _| match this.0.get_rate() {
                Ok(rate) => rate,
                _ => 0.,
            })
            .with_setter(|this: &mut TTS, _, v: f32| {
                let Features {
                    rate: rate_supported,
                    ..
                } = this.0.supported_features();
                if rate_supported {
                    let mut v = v;
                    if v < this.0.min_rate() {
                        v = this.0.min_rate();
                    } else if v > this.0.max_rate() {
                        v = this.0.max_rate();
                    }
                    this.0.set_rate(v).expect("Failed to set rate");
                }
            })
            .done();
        builder
            .property("min_rate")
            .with_getter(|this: &TTS, _| {
                let Features {
                    rate: rate_supported,
                    ..
                } = this.0.supported_features();
                if rate_supported {
                    this.0.min_rate()
                } else {
                    0.
                }
            })
            .done();
        builder
            .property("max_rate")
            .with_getter(|this: &TTS, _| {
                let Features {
                    rate: rate_supported,
                    ..
                } = this.0.supported_features();
                if rate_supported {
                    this.0.max_rate()
                } else {
                    0.
                }
            })
            .done();
        builder
            .property("normal_rate")
            .with_getter(|this: &TTS, _| {
                let Features {
                    rate: rate_supported,
                    ..
                } = this.0.supported_features();
                if rate_supported {
                    this.0.normal_rate()
                } else {
                    0.
                }
            })
            .done();
        builder
            .property("can_detect_screen_reader")
            .with_getter(|_: &TTS, _| cfg!(windows))
            .done();
        builder
            .property("has_screen_reader")
            .with_getter(|_, _| Tts::screen_reader_available())
            .done();
        builder
            .property("can_detect_is_speaking")
            .with_getter(|this: &TTS, _| {
                let Features {
                    is_speaking: is_speaking_supported,
                    ..
                } = this.0.supported_features();
                is_speaking_supported
            })
            .done();
        builder
            .property("is_speaking")
            .with_getter(|this: &TTS, _| {
                let Features {
                    is_speaking: is_speaking_supported,
                    ..
                } = this.0.supported_features();
                if is_speaking_supported {
                    this.0
                        .is_speaking()
                        .expect("Failed to determine if speaking")
                } else {
                    false
                }
            })
            .done();
        builder
            .signal("utterance_begin")
            .with_param_custom(SignalParam {
                name: "utterance".into(),
                default: Variant::default(),
                export_info: ExportInfo::new(VariantType::Object),
                usage: PropertyUsage::DEFAULT,
            })
            .done();
        builder
            .signal("utterance_end")
            .with_param_custom(SignalParam {
                name: "utterance".into(),
                default: Variant::default(),
                export_info: ExportInfo::new(VariantType::Object),
                usage: PropertyUsage::DEFAULT,
            })
            .done();
        builder
            .signal("utterance_stop")
            .with_param_custom(SignalParam {
                name: "utterance".into(),
                default: Variant::default(),
                export_info: ExportInfo::new(VariantType::Object),
                usage: PropertyUsage::DEFAULT,
            })
            .done();
    }

    #[method]
    fn set_language(&mut self, language: String) {
        if let Ok(voices) = self.0.voices() {
            let the_voice = voices.iter().find(|voice| { voice.language().as_str() == language });
            if let Some(the_voice) = the_voice {
                self.0.set_voice(the_voice).expect("Failed to set the new voice");
            } else {
                print!("No language available.");
            }
        }
    }

    #[method]
    fn speak(&mut self, message: String, interrupt: bool) -> Variant {
        if let Ok(id) = self.0.speak(message, interrupt) {
            let utterance: Instance<Utterance, Unique> = Instance::new();
            if id.is_some() {
                utterance
                    .map_mut(|u, _| u.0 = id)
                    .expect("Failed to set utterance ID");
            }
            utterance.owned_to_variant()
        } else {
            Variant::default()
        }
    }

    #[method]
    fn stop(&mut self) {
        self.0.stop().expect("Failed to stop");
    }

    #[method]
    fn is_rate_supported(&mut self) -> bool {
        let Features {
            rate: rate_supported,
            ..
        } = self.0.supported_features();
        rate_supported
    }

    #[method]
    fn are_utterance_callbacks_supported(&mut self) -> bool {
        let Features {
            utterance_callbacks: supported,
            ..
        } = self.0.supported_features();
        supported
    }

    #[method]
    fn _process(&mut self, #[base] base: &Node, _delta: f32) {
        if let Ok(msg) = self.1.try_recv() {
            match msg {
                Msg::UtteranceBegin(utterance_id) => {
                    let utterance: Instance<Utterance, Unique> = Instance::new();
                    utterance
                        .map_mut(|u, _| u.0 = Some(utterance_id))
                        .expect("Failed to set utterance ID");
                    base.emit_signal("utterance_begin", &[utterance.owned_to_variant()]);
                }
                Msg::UtteranceEnd(utterance_id) => {
                    let utterance: Instance<Utterance, Unique> = Instance::new();
                    utterance
                        .map_mut(|u, _| u.0 = Some(utterance_id))
                        .expect("Failed to set utterance ID");
                    base.emit_signal("utterance_end", &[utterance.owned_to_variant()]);
                }
                Msg::UtteranceStop(utterance_id) => {
                    let utterance: Instance<Utterance, Unique> = Instance::new();
                    utterance
                        .map_mut(|u, _| u.0 = Some(utterance_id))
                        .expect("Failed to set utterance ID");
                    base.emit_signal("utterance_stop", &[utterance.owned_to_variant()]);
                }
            }
        }
    }
}

struct TTSLibrary;

#[gdnative::init::callbacks]
impl GDNativeCallbacks for TTSLibrary {
    fn nativescript_init(handle: InitHandle) {
        env_logger::init();
        handle.add_tool_class::<Utterance>();
        handle.add_class::<TTS>();
    }
}
