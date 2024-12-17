def tabind():
    """
    Description: Given a word list of scrabble words, 
    find all the words containing letters in the string "TABIND" that only appear once.

    In this process, the txt file will be read and check every word in the txt file to see if it
    matches the constraints above.

    To do the constraint above, we will transform the letters inside of the word into a set
    because of the property that there is no duplicates.
    Ex. ABA = {"A", "B"}

    After that we will perform an intersection between the word_letters set and the tabind_letters set.
    Ex. ABA = {"A", "B"} ∩ {"T","A","B","I","N","D"} = {"A","B"}

    Once we do the intersection, we can compare the length of the the intersection and the len of the word
    and see that they do not match.
    Ex. len({"A","B"}) = 2, len("ABA") = 3. 
    """
    words = None
    tabind_letters={"T","A","B","I","N","D"}
    legal_words = set()

    with open("Collins_Scrabble_Words_2019.txt", "r") as file:
        words = file.read().splitlines()

    for word in words:
        word_letters = set(word)
        if len(word_letters.intersection(tabind_letters)) == len(word):
            legal_words.add(word)
    
    print(f"Legal Words: {sorted(legal_words)}")
    print(f"Length of Legal Words: {len(legal_words)}")

if __name__ == "__main__":
    tabind()