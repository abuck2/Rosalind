data = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC"
data = "TAACCCCGTCTCGATGACTGTTAAAAACCGTCAGGTGTGTAACATGCGACCATCGTGATATTACATCCATTCAAGGTAGTCGTGTTAGCCAGTAACTATCTATATACCATGCGCAGCGCACTGCATGAATTCCTTTGCTCCCGCTCAACCCTCAGTGTCCAACACTCCCATCCTGGCCGCCATTATAAATGTTACTGTGGCAACTAGTTACTATCACCGTACCAGGGTACCCAACACAACAACAATGAGGCCATTCAGCGAAATTAATCCTGGTCCTTCACATCCGCCTGTTCTTTGGGAGTCAGTTCGGCCGCTGCAACGCCTAGATCCGGGTCACAAAAAACTCGGCCAGAAGAGGACTAAACGCGAAAAAGGCATCCTTTATAAGTTGAGACCCGCCCCGAGTTCTTGACGGATCATCCAGACTTGGGGGTTCGGACCTTAGTTCATTGCAAGGATCCGTCCCGTCGTTATAAACTGGTCTGACGCCGAGCCACTTTGCCGTATCCAACAGCTGTCAGGTTTGATGTCCTCTATGATGCACGCGTCAGTTACGTCGCTAAGATGAATTATGAAGCCTGTCTGAATCGTCCCTACTGTACCGGAACCGTGACGAGGGAGAGTATCTACTAAACAGCCAGTACTAGTGTAATGCAAGTACCTAGAGGGGTATAAAAGATACATTTTTGACGGACCGTTATGACAGAATCCAGATGCCATTTTATACGGGTGGTCGTCGTTGCGCCTAACTTTCGGGACCACTCGACAGCGTTCTTTGGCTACCCCCTAATCGGAAACGCATCAAATCGCTAAATCTTTAAGCTTCGCTATCTCAATCACACTGACACGCCCGGGTCAATGATAGTAGACTTTTCTATAAAATCGGAGATGAC"

c_a = data.count('A')
c_c = data.count('C')
c_g = data.count('G')
c_t = data.count('T')

print("{} {} {} {}".format(c_a, c_c, c_g, c_t))
