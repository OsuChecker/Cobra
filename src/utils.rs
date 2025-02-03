use reqwest::Client;
use serde::Serialize;
use serde_json::Value;

/// Fonction générique pour effectuer des requêtes HTTP
/// - `method`: La méthode HTTP (`GET`, `POST`, etc.).
/// - `url`: L'URL du serveur.
/// - `body`: Optionnellement, un corps JSON à envoyer dans la requête.
/// Retourne un `Result` contenant la réponse ou une erreur.
pub fn request(method: &str, url: &str, body: Option<String>) -> Result<String, String> {
    let client = reqwest::blocking::Client::new();

    // Construction de la requête
    let request = match method {
        "POST" => client.post(url).header("Connection", "close"),
        "GET" => client.get(url).header("Connection", "close"),
        "PUT" => client.put(url),
        "DELETE" => client.delete(url),
        _ => return Err("Méthode non supportée".to_string()),
    };

    // Ajout du corps à la requête s'il est présent
    let request = if let Some(body_content) = body {
        request.body(body_content)
    } else {
        request
    };

    // Exécution de la requête
    let response = request
        .send()
        .map_err(|e| format!("Erreur réseau : {}", e))?;

    // Vérification si la réponse est un succès
    if response.status().is_success() {
        let text = response
            .text()
            .map_err(|e| format!("Erreur lors de la lecture de la réponse : {}", e))?;
        Ok(text)
    } else {
        Err(format!(
            "Erreur HTTP ({}): {}",
            response.status(),
            response.text().unwrap_or_default()
        ))
    }
}


pub fn request2(method: &str, url: &str, body: Option<String>) -> Result<String, String> {
    let client = reqwest::blocking::Client::new();

    // Construction de la requête
    let request = match method {
        "POST" => client.post(url).header("Connection", "close"),
        "GET" => client.get(url).header("Connection", "close"),
        "PUT" => client.put(url),
        "DELETE" => client.delete(url),
        _ => return Err("Méthode non supportée".to_string()),
    };

    // Ajout du corps à la requête s'il est présent
    let request = if let Some(body_content) = body {
        request.body(body_content)
    } else {
        request
    };

    // Exécution de la requête
    let response = request
        .send()
        .map_err(|e| format!("Erreur réseau : {}", e))?;

    // Vérification si la réponse est un succès
    if response.status().is_success() {
        let text = response
            .text()
            .map_err(|e| format!("Erreur lors de la lecture de la réponse : {}", e))?;
        Ok(text)
    } else {
        Err(format!(
            "Erreur HTTP ({}): {}",
            response.status(),
            response.text().unwrap_or_default()
        ))
    }
}


pub async fn request_async(
    method: &str,
    url: &str,
    body: Option<String>,
) -> Result<String, String> {
    
    let client = Client::new();

    // Construction de la requête
    let request = client.post(url);

    // Ajout du corps à la requête s'il est présent
    let request = if let Some(body_content) = body {
        request.body(body_content)
    } else {
        request
    };
    // Exécution de la requête et traitement de la réponse
    let response = request
        .send()
        .await
        .map_err(|e| format!("Erreur réseau : {}", e))?;
    // Vérification si la réponse est un succès
    if response.status().is_success() {
        let text = response
            .text()
            .await
            .map_err(|e| format!("Erreur lors de la lecture de la réponse : {}", e))?;
        Ok(text)
    } else {
        Err(format!(
            "Erreur HTTP ({}): {}",
            response.status(),
            response.text().await.unwrap_or_default()
        ))
    }
}

