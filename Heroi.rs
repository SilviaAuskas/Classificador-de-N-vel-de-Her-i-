pub struct Heroi {
    nome: String,
    xp: i32,
}

impl Heroi {
    pub fn new(nome: String, xp: i32) -> Heroi {
        Heroi { nome, xp }
    }

    pub fn get_nivel(&self) -> &str {
        match self.xp {
            0..=1000 => "Ferro",
            1001..=2000 => "Bronze",
            2001..=5000 => "Prata",
            5001..=7000 => "Ouro",
            7001..=8000 => "Platina",
            8001..=9000 => "Ascendente",
            9001..=10000 => "Imortal",
            _ => "Radiante",
        }
    }

    pub fn exibir_mensagem(&self) {
        println!("O Herói de nome *{}* está no nível de *{}*", self.nome, self.get_nivel());
    }
}

fn main() {
    let heroi = Heroi::new(String::from("Goku Heitor"), 7000);
    heroi.exibir_mensagem();
    let heroi = Heroi::new(String::from("Superman Fernando"), 8000);
    heroi.exibir_mensagem();
    let heroi = Heroi::new(String::from("SuperGirl Silvia"), 6000);
    heroi.exibir_mensagem();
    let heroi = Heroi::new(String::from("Hulk Haga"), 8000);
    heroi.exibir_mensagem();
}
