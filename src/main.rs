use std::vec::Vec;
use std::mem::swap;

use rand::Rng;

use minifb::{Key, Window, WindowOptions};

const PIERRE: u8 = 0;
const GRIS: u32 = 0xB0B0B0;
const CENDRES: u8 = 1;
const NOIR: u32 = 0x3B3737;
const JEUNE: u8 = 2;
const VERT: u32 = 0x68E180;
const ANCIEN: u8 = 3;
const VERT_FONCE: u32 = 0x24A609;
const DEBUT_FEU: u8 = 4;
const JAUNE: u32 = 0xF2F26E;
const FEU: u8 = 5;
const ORANGE: u32 = 0xE48F01;
const FIN_FEU: u8 = 6;
const ROUGE: u32 = 0xBA2E0B;

const M: usize = 600;
const N: usize = 600;
const TAILLE_ARBRE: usize = 2;

const HAUTEUR: usize = M * TAILLE_ARBRE;
const LARGEUR: usize = N * TAILLE_ARBRE;

struct Foret {
    m: usize,
    n: usize,
    arbres: Vec<Vec<u8>>,
    temp: Vec<Vec<u8>>,
}

impl Foret {
    fn init_alea(&mut self) {
        for i in 1..self.m - 1 {
            for j in 1..self.n - 1 {
                let e = rand::thread_rng().gen_range(PIERRE..=FIN_FEU);
                self.arbres[i][j] = e;
                self.temp[i][j] = e;
            }
        }
    }

    fn init_etat(&mut self, etat: u8) {
        for i in 1..self.m - 1 {
            for j in 1..self.n - 1 {
                self.arbres[i][j] = etat;
                self.temp[i][j] = etat;
            }
        }
    }

    fn echanger_arbres(&mut self) {
        swap(&mut self.arbres, &mut self.temp);
    }

    fn recuperer_voisins(&self, i: usize, j: usize) -> Vec<u8> {
        let voisins = vec![
            self.arbres[i-1][j-1],
            self.arbres[i-1][j],
            self.arbres[i-1][j+1],
            self.arbres[i][j-1],
            self.arbres[i][j+1],
            self.arbres[i+1][j-1],
            self.arbres[i+1][j],
            self.arbres[i+1][j+1], 
        ];
        voisins
    }

    fn nouvel_etat(&self, i: usize, j: usize) -> u8 {
        let mut rng = rand::thread_rng();
        match self.arbres[i][j] {
            PIERRE => PIERRE,
            CENDRES => if rng.gen::<f64>() < 0.001 {JEUNE} else {CENDRES},
            JEUNE => {
                let voisins = self.recuperer_voisins(i, j);
                if rng.gen::<f64>() < 0.01 && voisins.contains(&DEBUT_FEU) {
                    DEBUT_FEU
                } else if rng.gen::<f64>() < 0.02 && voisins.contains(&FEU) {
                    DEBUT_FEU
                } else if rng.gen::<f64>() < 0.01 && voisins.contains(&FIN_FEU) {
                    DEBUT_FEU
                } else if rng.gen::<f64>() < 0.005 {
                    ANCIEN
                } else {
                    JEUNE
                }
            },
            ANCIEN => {
                let voisins = self.recuperer_voisins(i, j);
                if rng.gen::<f64>() < 0.1 && voisins.contains(&DEBUT_FEU) {
                    DEBUT_FEU
                } else if rng.gen::<f64>() < 0.2 && voisins.contains(&FEU) {
                    DEBUT_FEU
                } else if rng.gen::<f64>() < 0.1 && voisins.contains(&FIN_FEU) {
                    DEBUT_FEU
                } else if rng.gen::<f64>() < 0.00005 && voisins.iter().filter(|&n| *n == ANCIEN).count() >= 5 {
                    DEBUT_FEU
                } else {
                    ANCIEN
                }
            },
            DEBUT_FEU => if rng.gen::<f64>() < 0.1 {FEU} else {DEBUT_FEU},
            FEU => if rng.gen::<f64>() < 0.1 {FIN_FEU} else {FEU},
            FIN_FEU => if rng.gen::<f64>() < 0.1 {CENDRES} else {FIN_FEU},
            _ => PIERRE,
        }
    }

    fn iteration_suivante(&mut self) {
        for i in 1..self.m - 1 {
            for j in 1..self.n - 1 {
                self.temp[i][j] = self.nouvel_etat(i, j);
            }
        }
        self.echanger_arbres();
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; LARGEUR * HAUTEUR];
    
    let mut window = Window::new(
        "Jeu de la vie - ESC pour quitter",
        LARGEUR,
        HAUTEUR,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut foret = Foret {
        m: M,
        n: N,
        arbres: vec![vec![0u8; LARGEUR]; HAUTEUR],
        temp: vec![vec![0u8; LARGEUR]; HAUTEUR],
    };
    foret.init_etat(CENDRES);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        foret.iteration_suivante();

        for (index, cell) in buffer.iter_mut().enumerate() {
            let x = index / LARGEUR;
            let y = index % LARGEUR;

            let cell_x = x / TAILLE_ARBRE;
            let cell_y = y / TAILLE_ARBRE;

            *cell = match foret.arbres[cell_x][cell_y] {
                PIERRE => GRIS,
                CENDRES => NOIR,
                JEUNE => VERT,
                ANCIEN => VERT_FONCE,
                DEBUT_FEU => JAUNE,
                FEU => ORANGE,
                FIN_FEU => ROUGE,
                _ => GRIS,
            };
        }
        window.update_with_buffer(&buffer, LARGEUR, HAUTEUR).unwrap();
    }
}
