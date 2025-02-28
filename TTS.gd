tool
extends Node

signal utterance_begin(utterance)

signal utterance_end(utterance)

signal utterance_stop(utterance)

var TTS

var tts


func _init():
	if OS.get_name() == "Server" or OS.has_feature("JavaScript"):
		return
	elif Engine.has_singleton("GodotTTS"):
		tts = Engine.get_singleton("GodotTTS")
	else:
		TTS = preload("res://addons/godot-tts/godot-tts.gdns")
	if TTS and (TTS.can_instance() or Engine.editor_hint):
		tts = TTS.new()
	if tts:
		if not tts is JNISingleton:
			self.add_child(tts)
		if self.are_utterance_callbacks_supported:
			tts.connect("utterance_begin", self, "_on_utterance_begin")
			tts.connect("utterance_end", self, "_on_utterance_end")
			tts.connect("utterance_stop", self, "_on_utterance_stop")
		print_debug("TTS available!")
	else:
		print_debug("TTS not available!")


func _ready():
	pause_mode = Node.PAUSE_MODE_PROCESS


func _get_min_volume():
	if OS.has_feature('JavaScript'):
		return 0
	else:
		return 0

var min_volume setget , _get_min_volume


func _get_max_volume():
	if OS.has_feature('JavaScript'):
		return 1.0
	else:
		return 1.0

var max_volume setget , _get_max_volume


func _get_normal_volume():
	if OS.has_feature('JavaScript'):
		return 1.0
	else:
		return 0

var normal_volume setget , _get_normal_volume

var javascript_volume = self.normal_volume


func _set_volume(volume):
	if volume < self.min_volume:
		volume = self.min_volume
	elif volume > self.max_volume:
		volume = self.max_volume
	if Engine.has_singleton("GodotTTS"):
		tts.set_volume(volume)
	elif tts != null:
		tts.volume = volume
	if OS.has_feature('JavaScript'):
		javascript_volume = volume


func _get_volume():
	if OS.has_feature('JavaScript'):
		return javascript_volume
	else:
		return 0


var volume setget _set_volume, _get_volume


func _get_volume_percentage():
	return range_lerp(self.volume, self.min_volume, self.max_volume, 0, 100)


func _set_volume_percentage(v):
	self.rate = range_lerp(v, 0, 100, self.min_volume, self.max_volume)


var volume_percentage setget _set_volume_percentage, _get_volume_percentage


func _get_normal_volume_percentage():
	return range_lerp(self.normal_volume, self.min_volume, self.max_volume, 0, 100)

var normal_volume_percentage setget , _get_normal_volume_percentage


func _get_min_rate():
	if OS.has_feature('JavaScript'):
		return 0.1
	elif Engine.has_singleton("GodotTTS"):
		return 0.1
	elif tts != null:
		return tts.min_rate
	else:
		return 0


var min_rate setget , _get_min_rate


func _get_max_rate():
	if OS.has_feature('JavaScript'):
		return 10.0
	elif Engine.has_singleton("GodotTTS"):
		return 10.0
	elif tts != null:
		return tts.max_rate
	else:
		return 0


var max_rate setget , _get_max_rate


func _get_normal_rate():
	if OS.has_feature('JavaScript'):
		return 1.0
	elif Engine.has_singleton("GodotTTS"):
		return 1.0
	elif tts != null:
		return tts.normal_rate
	else:
		return 0


var normal_rate setget , _get_normal_rate

var javascript_rate = self.normal_rate


func _set_rate(rate):
	if rate < self.min_rate:
		rate = self.min_rate
	elif rate > self.max_rate:
		rate = self.max_rate
	if Engine.has_singleton("GodotTTS"):
		return tts.set_rate(rate)
	elif tts != null:
		tts.rate = rate
	elif OS.has_feature('JavaScript'):
		javascript_rate = rate


func _get_rate():
	if Engine.has_singleton("GodotTTS"):
		return tts.get_rate()
	elif tts != null:
		return tts.rate
	elif OS.has_feature('JavaScript'):
		return javascript_rate
	else:
		return 0


var rate setget _set_rate, _get_rate


func _get_rate_percentage():
	return range_lerp(self.rate, self.min_rate, self.max_rate, 0, 100)


func _set_rate_percentage(v):
	self.rate = range_lerp(v, 0, 100, self.min_rate, self.max_rate)


var rate_percentage setget _set_rate_percentage, _get_rate_percentage


func _get_normal_rate_percentage():
	return range_lerp(self.normal_rate, self.min_rate, self.max_rate, 0, 100)


func _get_speed():
	if Engine.has_singleton("GodotTTS"): # Android
		return self.rate
	elif tts != null:                    # iOS
		return self.rate / (self.normal_rate * 0.8)
	elif OS.has_feature('JavaScript'):   # Web
		return self.rate / (self.normal_rate * 0.9)
	else:
		return 0


func _set_speed(v):
	if Engine.has_singleton("GodotTTS"): # Android
		self.rate = v
	elif tts != null:                    # iOS
		self.rate = v * self.normal_rate * 0.8
	elif OS.has_feature('JavaScript'):   # Web
		self.rate = v * self.normal_rate * 0.9


var speed setget _set_speed, _get_speed


var normal_rate_percentage setget , _get_rate_percentage
var javascript_lang = "en-US"

func set_language(language):
	var lang_id = "en-US"
	if language == "en":
		lang_id = "en-US"
	elif language == "fr":
		lang_id = "fr-FR"
	elif language == "es":
		lang_id = "es-ES"
	elif language == "ko":
		lang_id = "ko-KR"

	if tts != null:
		tts.set_language(lang_id)
	elif OS.has_feature('JavaScript'):
		javascript_lang = lang_id


func speak(text, interrupt := true):
	var utterance
	if tts != null:
		utterance = tts.speak(text, interrupt)
	elif OS.has_feature('JavaScript'):
		var code = (
			"""
			let utterance = new SpeechSynthesisUtterance("%s")
			utterance.voice = window.speechSynthesis.getVoices().find((v) => v.lang === "%s")
			utterance.rate = %s
			utterance.volume = %s
		"""
			% [text.replace("\n", " "), javascript_lang, javascript_rate, javascript_volume]
		)
		if interrupt:
			code += """
				window.speechSynthesis.cancel()
			"""
		code += "window.speechSynthesis.speak(utterance)"
		JavaScript.eval(code)
	else:
		print_debug("%s: %s" % [text, interrupt])
	return utterance


func stop():
	if tts != null:
		tts.stop()
	elif OS.has_feature('JavaScript'):
		JavaScript.eval("window.speechSynthesis.cancel()")


func _get_is_rate_supported():
	if Engine.has_singleton("GodotTTS"):
		return true
	elif OS.has_feature('JavaScript'):
		return true
	elif tts != null:
		return tts.is_rate_supported()
	else:
		return false


var is_rate_supported setget , _get_is_rate_supported


func _get_are_utterance_callbacks_supported():
	if Engine.has_singleton("GodotTTS"):
		return true
	elif OS.has_feature('JavaScript'):
		return false
	elif tts != null:
		return tts.are_utterance_callbacks_supported()
	else:
		return false


var are_utterance_callbacks_supported setget , _get_are_utterance_callbacks_supported


func _get_can_detect_is_speaking():
	if Engine.has_singleton("GodotTTS"):
		return true
	elif OS.has_feature('JavaScript'):
		return true
	elif tts != null:
		return tts.can_detect_is_speaking
	return false


var can_detect_is_speaking setget , _get_can_detect_is_speaking


func _get_is_speaking():
	if Engine.has_singleton("GodotTTS"):
		return tts.is_speaking()
	elif OS.has_feature('JavaScript'):
		return JavaScript.eval("window.speechSynthesis.speaking")
	elif tts != null:
		return tts.is_speaking
	return false


var is_speaking setget , _get_is_speaking


func _get_can_detect_screen_reader():
	if Engine.has_singleton("GodotTTS"):
		return true
	elif OS.has_feature('JavaScript'):
		return false
	elif tts != null:
		return tts.can_detect_screen_reader
	return false


var can_detect_screen_reader setget , _get_can_detect_screen_reader


func _get_has_screen_reader():
	if Engine.has_singleton("GodotTTS"):
		return tts.has_screen_reader()
	elif OS.has_feature('JavaScript'):
		return false
	elif tts != null:
		return tts.has_screen_reader
	return false


var has_screen_reader setget , _get_has_screen_reader


func singular_or_plural(count, singular, plural):
	if count == 1:
		return singular
	else:
		return plural


func _on_utterance_begin(utterance):
	emit_signal("utterance_begin", utterance)


func _on_utterance_end(utterance):
	emit_signal("utterance_end", utterance)


func _on_utterance_stop(utterance):
	emit_signal("utterance_stop", utterance)


func _exit_tree():
	if not tts or not TTS:
		return
	tts.free()
