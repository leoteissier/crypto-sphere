use super::Chiffrement;

pub struct RailFence {
    pub rails: usize,
}

impl Chiffrement for RailFence {
    fn chiffrer(&self, texte: &str) -> String {
        let mut fence: Vec<Vec<char>> = vec![vec![]; self.rails];
        let mut rail = 0;
        let mut direction = 1;

        for c in texte.chars() {
            fence[rail].push(c);
            rail = (rail as isize + direction) as usize;

            if rail == 0 || rail == self.rails - 1 {
                direction *= -1;
            }
        }

        fence.into_iter().flatten().collect()
    }

    fn dechiffrer(&self, texte: &str) -> String {
        let mut fence: Vec<Vec<Option<char>>> = vec![vec![None; texte.len()]; self.rails];
        let mut rail = 0;
        let mut direction = 1;

        for i in 0..texte.len() {
            fence[rail][i] = Some('*');
            rail = (rail as isize + direction) as usize;

            if rail == 0 || rail == self.rails - 1 {
                direction *= -1;
            }
        }

        let mut idx = 0;
        for row in &mut fence {
            for elem in row.iter_mut() {
                if elem.is_some() {
                    *elem = Some(texte.chars().nth(idx).unwrap());
                    idx += 1;
                }
            }
        }

        let mut result = String::new();
        rail = 0;
        direction = 1;

        for i in 0..texte.len() {
            result.push(fence[rail][i].unwrap());
            rail = (rail as isize + direction) as usize;

            if rail == 0 || rail == self.rails - 1 {
                direction *= -1;
            }
        }

        result
    }
}
