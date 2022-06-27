import numpy as np

PATH = "C:\\Users\\menno\\Desktop\\Code\\Rust\\hangman\\"

easy = []
medium = []
hard = []

length = dict()
set_length = dict()

with open(PATH + "words.txt", "r") as file:
    data = file.read().splitlines()
    for word in data:
        word_as_set = set(el for el in word)
        try:
            length[str(len(word))] += 1
        except:
            length.setdefault(str(len(word)), 1)

        try:
            set_length[str(len(word_as_set))] += 1
        except:
            set_length.setdefault(str(len(word_as_set)), 1)

        if (len(word) > 6 and len(word_as_set) < 4) or (
            len(word) < 3 and len(word_as_set) < 3
        ):
            easy.append(word)
        elif len(word) > 4 and len(word_as_set) < 3:
            medium.append(word)
        else:
            hard.append(word)

print(f"easy: {len(easy)}")
print(f"med: {len(medium)}")
print(f"hard: {len(hard)}")

l = np.zeros(17)
for size, number in length.items():
    l[int(size)] = number

ls = np.zeros(12)
for size, number in set_length.items():
    ls[int(size)] = number

print(l)
print(ls)
