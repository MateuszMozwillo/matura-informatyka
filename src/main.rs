fn main() {
    println!("Hello, world!");
    let bin_txt = include_str!("../dane-2023/bin_przyklad.txt");
    let dane_bin_vec: Vec<&str> = bin_txt.split_whitespace().collect();
    dbg!(zadanie_2_2(&dane_bin_vec));
    dbg!(bin_to_int("1111101001001010101"));
    dbg!(zadanie_2_3(&dane_bin_vec));
}

fn zadanie_2_2(dane: &Vec<&str>) -> i32 {
    let mut wynik = 0;

    for i in dane {
        let mut bloki = 0;
        let mut last = ' ';
    
        for j in i.chars() {
            if j != last {
                bloki += 1;
            }
            last = j;
        }
    
        if bloki <= 2 {
            wynik += 1;
        }
    }
    
    wynik
}

fn zadanie_2_3(dane: &Vec<&str>) -> String {
    let mut max = "";
    
    for i in dane {
        if bin_to_int(i) > bin_to_int(max) {
            max = i;
        }
    }

    max.to_string()
}

fn zadanie_2_5(dane: &Vec<&str>) -> Vec<String> {
    let mut wyniki: Vec<String> = vec![];
    
    for i in dane {

    }

    wyniki
}

fn bin_to_int(bin: &str) -> i32 {
    let mut wynik = 0;
    for i in bin.chars() {
        if i == '1' {
            wynik = wynik << 1;
            wynik += 1;
            
        }
        if i == '0' {
            wynik = wynik << 1;
        }
    }
    wynik
}

fn int_to_bin(int: i32) -> String {
    let mut wynik = "asd".to_string();

    wynik
}
