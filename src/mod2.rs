// Mod para função do segundo grau
#[allow(unused)]

pub fn f2_deg(a: f64, b: f64, c: f64) -> () {
    // Função de segundo grau
    let delta: f64 = b.powi(2) - 4.0 * a * c;

    if delta < 0.0 {
        println!("Não há raízes reais");
    } else if delta == 0.0 {
        let x: f64 = -b / (2.0 * a);
        println!("Há uma raiz real: {}", x);
    } else {
        let x1: f64 = (-b + delta.sqrt()) / (2.0 * a);
        let x2: f64 = (-b - delta.sqrt()) / (2.0 * a);
        println!("Há duas raízes reais: {} e {}", x1, x2);
    };
    let vertex_x: f64 = -b / (2.0 * a);
    let vertex_y: f64 = -delta / (4.0 * a);
    println!("O vértice da parábola é: ({}, {})", vertex_x, vertex_y);
}
