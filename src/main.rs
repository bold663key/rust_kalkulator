use std::io;
use std::fmt::Write;

fn main() {

    let mut kalk = String::new();

    write!(kalk, "

Witaj w kalkulatorze rust!
     ____________
    |  ________  |
    | |12345678| |
    | ---------- |
    | [7|8|9][+] |
    | [4|5|6][-] |
    | [1|2|3][*] |
    | [ |O|=][/] |
    |____________|
    
    ").unwrap();
    
    println!("{kalk}");
    

loop{
    println!("Wpisz pierwszą cyfrę: ");
    
    let mut cyfra_1 = String::new();

    io::stdin()
        .read_line(&mut cyfra_1)
        .expect("Nie udało się odczytać cyfry");

    let cyfra_1: i128 = match cyfra_1.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("Wpisz drugą cyfrę: ");
    
    let mut cyfra_2 = String::new();

    io::stdin()
        .read_line(&mut cyfra_2)
        .expect("Nie udało się odczytać cyfry");

    let cyfra_2: i128 = match cyfra_2.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("-----------------------------------------------");
    println!("Wybierz typ działania (wpisz cyfrę od 1 do 4): ");
    println!("-----------------------------------------------");
    println!("1.SUMA +");
    println!("2.RÓŻNICA -");
    println!("3.ILOCZYN *");
    println!("4.ILORAZ /");
    println!("-----------------------------------------------");
    println!("5.WYŁĄCZ PROGRAM");
    println!("-----------------------------------------------");

    let mut odp = String::new();

    io::stdin()
        .read_line(&mut odp)
        .expect("Nie udało się odczytać cyfry");

    let odp: i32 = match odp.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };
    if odp == 1{
        let suma: i128 = cyfra_1 + cyfra_2;
        println!("-----------------------------------------------");
        println!("Suma wynosi: {suma}");
        println!("-----------------------------------------------");
    }else if odp == 2{
        let roznica: i128 = cyfra_1 - cyfra_2;
        println!("-----------------------------------------------");
        println!("Różnica wynosi: {roznica}");
        println!("-----------------------------------------------");
    }else if odp == 3{
        let iloczyn: i128 = cyfra_1 * cyfra_2;
        println!("-----------------------------------------------");
        println!("Iloczyn wynosi: {iloczyn}");
        println!("-----------------------------------------------");
    }else if odp == 4{
        let iloraz: i128 = cyfra_1 / cyfra_2;
        println!("-----------------------------------------------");
        println!("Iloraz wynosi: {iloraz}");
        println!("-----------------------------------------------");
    }else if odp == 5{
        println!("Do zobaczenia!");
        break;
    }else {
        println!("Nie udało się odczytać cyfry");
    }
}
}