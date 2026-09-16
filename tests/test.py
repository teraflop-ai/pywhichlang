from pywhichlang import detect_language, detect_languages

print(detect_language("This sentence is written in English."))
print(detect_languages([
    "This sentence is written in English.",
    "Cette phrase est écrite en français.",
]))