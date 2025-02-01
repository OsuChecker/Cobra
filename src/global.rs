use crate::structs::{Arm, OutputValues};
use lazy_static::lazy_static;
use std::sync::Mutex;

// Déclare la variable globale
lazy_static! {
    static ref TOKEN: Mutex<Option<String>> = Mutex::new(None);
}

// Fonction pour mettre à jour le token
pub fn set_token(new_token: String) {
    let mut token = TOKEN.lock().unwrap();
    *token = Some(new_token);
}

// Fonction pour récupérer le token
pub fn get_token() -> Option<String> {
    let token = TOKEN.lock().unwrap();
    token.clone()
}
