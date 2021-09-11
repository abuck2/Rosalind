use std::fs;
use std::collections::btree_map::BTreeMap;

pub fn looping(){
    let mut begin=4658;
    let ending=8734;
    let mut accum:i64=0;
    while begin < ending{
        if begin%2==1{
            accum=accum+begin;
        }
        begin=begin+1;
    }
    loop //like while true
        {
            accum *= 2;
            if accum > 120 {
                break;
            }
        }
    println!("Accum = {}", accum);
}

pub fn file_opener(){
    let filename = "C:\\Users\\Alexis\\Documents\\a imprimer\\test\\rosalind_ini5.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut content_split :Vec<&str> = contents.split('\n').collect();

    let mut i :i64 = 1;
    for line in &mut content_split {
        if i%2==0{
            println!("{}", line);
        }
        i += 1;
    }
}

pub fn count_nucleosine(){
    let dna_strand = "TCTTCCTTCCGTAAGGGTTGTAGTGCACTCCCAGAGCACCACTTGGTTATCGTGCACTGTGCTAAGC";
    let mut count = BTreeMap::new();
    for elem in dna_strand.chars(){
        *count.entry(elem).or_insert(0) += 1;
    };
    println!("Number of occurences per character");
    for (ch, count) in &count {
        println!("{}", count);
    }
}

pub fn rna_transcription(){
    let mut dna_strand = str::replace("TTCGGTCCTCGAATCAATCCCCCTGCCGTAAAGGTATGACTGCTTATATGTACCGTTAGCGTTTCGTTAGTCCAATGAGCCGAACTTAGCACAACCCACTATTACCTGCCTGCCGACCGGCAGTCGACTAGTAGTCGTCGCATTCCCCATTAAGCTTGGTTCCTGAAAAGCCCACTTGGTGCCTCCGTCATGATACAGGCTGTGGGTCGGCGGCAATGAAATTGAGTGTCCGCTTAATAGGCCATCCACTTGCCTCGGTGATTCCGTTCGTGGGGTGCGCAATATACAGGATGATAACCTACTGAGCGCTAGTCTCGCGGGGTTAGAGCAGTCCCACATTCCAGGTCAATCCAGGTACGGAGCCATACATAGTAACCCACCCAACTTAGGTGCCGTTTGGTCAAGGAGATCCAACAGTGTTTGCCATTTGAGATCTGGTAGTCGCAGCCCCTCCCTATCTTGGACAACTTCGCGGTTTAACTTAAAATCGGGGGCCCTGATTTTTTAATTTCCAGCGCGGATCCGGGGGGCAATCTTTGTATATGCTGGGCCAACCCCTGCGCGCGGAGGGTGCTTCGTCGACCATCGTCAATCCTAGACCGCCCGGGTAGCGGACCGAATGAGAGACATACCCTGAAATAATATGATATAGCCGATTTTGATGAAACCCCATTCTGAGACGTTTTATTTAGCTTATGGCTTGGGCACTTAGACAGTTTGATAACAATGCGACGCGTACTACACATTTCCTCGGTACTTGCAATAGAAATTGGACACCTCGAGGACACGGACCTCATGGGCGATACTTCCTTGGCCGCGGCGTACACCATGGCGTCTCTTGATATACTCCAGTTATATGCGGGTGGGTCCACGGACTTGGAGTGGCGCTCTGGTCATTCAGAGTCCATTCTAAGTGGGCGTCGCCCTGGCGCCCCTGGATATACAGATTTCATGGGTCAGGAGGA", "T", "U");
    dna_strand.replace("T", "U");
    println!("{}", dna_strand);
}
