// Clé par défaut pour le chiffre de César
pub const CLE_CESAR: i32 = 3;

// Clé par défaut pour le chiffrement XOR (un caractère ASCII)
pub const CLE_XOR: u8 = b'A';

// Clé par défaut pour le chiffre de Vigenère
pub const CLE_VIGENERE: &str = "CRYPTO";

// Clé par défaut pour le chiffre de Substitution (26 lettres uniques)
pub const CLE_SUBSTITUTION: &str = "QWERTYUIOPLKJHGFDSAZXCVBNM";

// Clé par défaut pour Rail Fence (nombre de rails)
pub const CLE_RAIL_FENCE: usize = 3;
