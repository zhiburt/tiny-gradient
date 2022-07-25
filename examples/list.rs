use tiny_gradient::{Gradient, GradientStr, RGB};

const LOGO: &str = r"
     _     _                                                       _   _                  _   
    | |   (_)                                                     | | (_)                | |  
    | |_   _   _ __    _   _   ______    __ _   _ __    __ _    __| |  _    ___   _ __   | |_ 
    | __| | | | '_ \  | | | | |______|  / _` | | '__|  / _` |  / _` | | |  / _ \ | '_ \  | __|
    | |_  | | | | | | | |_| |          | (_| | | |    | (_| | | (_| | | | |  __/ | | | | | |_ 
     \__| |_| |_| |_|  \__, |           \__, | |_|     \__,_|  \__,_| |_|  \___| |_| |_|  \__|
                        __/ |            __/ |                                                
                       |___/            |___/                                                 
";

fn main() {
    let raibow = [
        RGB::new(255, 0, 0),
        RGB::new(0, 255, 0),
        RGB::new(0, 0, 255),
    ];
    println!("{}", LOGO.gradient(raibow));

    [
        Gradient::Atlast,
        Gradient::Cristal,
        Gradient::Fruit,
        Gradient::Instagram,
        Gradient::Mind,
        Gradient::Morning,
        Gradient::Passion,
        Gradient::Pastel,
        Gradient::Rainbow,
        Gradient::Retro,
        Gradient::Summer,
        Gradient::Teen,
        Gradient::Vice,
        Gradient::Monsoon,
        Gradient::Forest,
    ]
    .into_iter()
    .for_each(|gradient| {
        let short = "▇".repeat(10);
        let short = short.gradient(gradient);
        let long = "▇".repeat(100);
        let long = long.gradient(gradient);
        let name = format!("{:?}", gradient);
        println!(" {:<15} {} {}", name, short, long);
    });
}
