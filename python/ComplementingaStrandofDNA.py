#data = "AAAACCCGGT"
data = input()

data = data.replace('A', 't').replace('T', 'a').replace('C', 'g').replace('G', 'c').upper()[::-1]

print(data)
