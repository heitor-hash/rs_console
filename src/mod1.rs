use ibig::UBig;

#[allow(unused)]

pub fn fibonacci(index: u32) -> () {
    if index > 1_500_000u32 {
        println!("O número é maior que 1.500.000, o programa não roda, ok?");
        return;
    } else if index > 400_000u32 {
        println!("Processando...");
    }

    let mut a: UBig = UBig::from(0u8);
    let mut b: UBig = UBig::from(1u8);

    if index == 0u32 {
        println!("fibonacci({}) = {}", index, a);
        return;
    } else if index == 1u32 {
        println!("fibonacci({}) = {}", index, b);
        return;
    } else {
        for _ in 0..index {
            let t = &a + &b;
            a = b.clone();
            b = t;
        }

        println!("fibonacci({}) = {}", index, b);
    }
}
