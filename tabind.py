words = None
tabind_letters={"T","A","B","I","N","D"}
legal_words = set()

with open("Collins Scrabble Words (2019).txt", "r") as file:
    words = set(file.read().splitlines())

for word in words:
    word_letters = set(word)
    if len(word_letters.intersection(tabind_letters)) != len(word):
        continue
    legal_words.add(word)
    
print(f"Legal Words: {sorted(legal_words)}")
print(f"Length of Legal Words: {len(legal_words)}")