use druid::text::format::{Formatter, Validation, ValidationError};
use druid::text::Selection;
use druid::{
    widget::{Button, Flex, Label, TextBox},
    AppLauncher, Data, Env, Lens, LensExt, Target, Widget, WidgetExt, WindowDesc,
};
use druid::{AppDelegate, Command, DelegateCtx, Handled, Selector};
use std::fmt;
use tokio::runtime::Runtime;

// État de l'application partagé
#[derive(Clone, Data, Lens)]
struct AppState {
    view: AppView,
    login_state: LoginState,
    register_state: RegisterState,
}

// État pour définir quelle vue est affichée (Login ou Register)
#[derive(Clone, Data, PartialEq)]
enum AppView {
    Login,
    Register,
    Connected,
}

// Modèle de données pour la page de connexion
#[derive(Clone, Data, Lens)]
struct LoginState {
    username: String,
    password: String,
    status: String,
}

// Modèle de données pour la page d'inscription
#[derive(Clone, Data, Lens)]
struct RegisterState {
    username: String,
    password: String,
    repeat_password: String,
    status: String,
}

// Créez un délégué pour gérer les commandes globales
pub struct Delegate;

impl druid::AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,                  // Self mutable car on pourrait modifier l'état du Delegate
        _: &mut druid::DelegateCtx, // &mut druid::DelegateCtx : contexte du delegate
        _: druid::Target,           // druid::Target : cible de l'action
        cmd: &druid::Command,       // &druid::Command : la commande reçue
        data: &mut AppState,        // &mut AppState : état global qui peut être modifié
        _: &druid::Env,             // &druid::Env : environnement global
    ) -> druid::Handled {
        // retourne `druid::Handled`, indiquant si la commande est prise en charge ou non
        // Vérifiez si la commande est un succès de connexion
        if cmd.is(druid::Selector::<()>::new("login.success")) {
            // Passez à la vue "Connected"
            data.view = AppView::Connected;
            return druid::Handled::Yes; // Commande traitée
        }

        // Vérifiez si la commande est une erreur de connexion
        if cmd.is(druid::Selector::<()>::new("login.success")) {
            // Affichez un message d'erreur
            data.login_state.status = "Erreur lors de la connexion.".to_string();
            return druid::Handled::Yes; // Commande traitée
        }

        // Sinon, laissez l'événement non traité
        druid::Handled::No
    }
}

// Construire la page de connexion
fn build_login_ui() -> impl Widget<AppState> {
    let username_label = Label::new("Username :");
    let username_input = TextBox::new().lens(AppState::login_state.then(LoginState::username));

    let password_label = Label::new("Password :");
    let password_input = TextBox::new().lens(AppState::login_state.then(LoginState::password));

    let status_label =
        Label::new(|data: &AppState, _: &Env| format!("{}", data.login_state.status))
            .with_text_size(12.0);

    let login_button = Button::new("Se connecter").on_click(|ctx, data: &mut AppState, _| {
        let username = data.login_state.username.clone();
        let password = data.login_state.password.clone();

        // Prépare le corps de la requête
        let body =
            Some(format!(r#"{{"uuid": "{}","password": "{}"}}"#, username, password).to_string());

        // Récupérer le `EventSink` pour communiquer avec l'UI
        let event_sink = ctx.get_external_handle();

        // Lancer une tâche asynchrone avec `tokio::spawn`
        tokio::spawn(async move {
            // Appeler la fonction qui effectue la requête HTTP
            let result =
                crate::utils::request_async("POST", "https://osef.me/api/login", body).await;

            match result {
                Ok(response) => {
                    // Si la connexion est réussie, enregistrer le token global
                    crate::global::set_token(response);

                    // Notifier l'interface que la vue doit changer à "Connected"
                    event_sink
                        .submit_command(
                            druid::Selector::new("login.success"), // Identifiant unique pour la commande
                            (),
                            druid::Target::Auto, // Cible automatique
                        )
                        .expect("Impossible d'envoyer le changement de vue");
                }
                Err(_) => {
                    // Si une erreur survient, informer l'interface
                    event_sink
                        .submit_command(
                            druid::Selector::new("login.error"),
                            (),
                            druid::Target::Auto,
                        )
                        .expect("Impossible d'envoyer l'erreur de connexion");
                }
            }
        });
    });

    let register_button = Button::new("S'inscrire").on_click(|_, data: &mut AppState, _| {
        data.view = AppView::Register;
    });

    Flex::column()
        .with_child(username_label)
        .with_child(username_input.padding(5.0))
        .with_child(password_label)
        .with_child(password_input.padding(5.0))
        .with_child(login_button.padding(10.0))
        .with_child(register_button.padding(10.0))
        .with_child(status_label.padding(10.0))
}

// Construire la page d'enregistrement
fn build_register_ui() -> impl Widget<AppState> {
    let username_label = Label::new("Nom d'utilisateur :");
    let username_input = TextBox::new()
        .with_placeholder("Entrez votre nom")
        .lens(AppState::register_state.then(RegisterState::username));

    let password_label = Label::new("Mot de passe :");
    let password_input = TextBox::new()
        .with_placeholder("Entrez votre mot de passe")
        .lens(AppState::register_state.then(RegisterState::password));

    let repeat_password_label = Label::new("Répétez le mot de passe :");
    let repeat_password_input = TextBox::new()
        .with_placeholder("Répétez le mot de passe")
        .lens(AppState::register_state.then(RegisterState::repeat_password));

    let status_label =
        Label::new(|data: &AppState, _: &Env| format!("{}", data.register_state.status))
            .with_text_size(12.0);

    let register_button = Button::new("S'inscrire").on_click(|ctx, data: &mut AppState, _| {
        let register_data = &mut data.register_state;

        if register_data.password != register_data.repeat_password {
            register_data.status = "Les mots de passe ne correspondent pas".to_string();
        } else {
            let username = register_data.username.clone();
            let password = register_data.password.clone();

            // Prépare le corps de la requête
            let body = Some(
                format!(r#"{{"uuid": "{}","password": "{}"}}"#, username, password).to_string(),
            );

            // Récupérer le `EventSink` pour communiquer avec l'UI
            let event_sink = ctx.get_external_handle();

            // Lancer une tâche asynchrone avec `tokio::spawn`
            tokio::spawn(async move {
                // Appeler la fonction qui effectue la requête HTTP pour s'inscrire
                let result =
                    crate::utils::request_async("POST", "https://osef.me/api/register", body).await;

                match result {
                    Ok(response) => {
                        // Si l'inscription est réussie, enregistrer le token global
                        crate::global::set_token(response);

                        // Notifier l'interface que l'inscription est réussie et passer à la vue connectée
                        event_sink
                            .submit_command(
                                druid::Selector::new("login.success"), // On peut réutiliser le même sélecteur
                                (),
                                druid::Target::Auto,
                            )
                            .expect("Impossible d'envoyer le changement de vue");
                    }
                    Err(_) => {
                        // En cas d'erreur, notifier l'interface
                        event_sink
                            .submit_command(
                                druid::Selector::new("login.error"), // Commande pour gérer l'erreur
                                (),
                                druid::Target::Auto,
                            )
                            .expect("Impossible d'envoyer l'erreur d'inscription");
                    }
                }
            });
        }
    });

    let back_button = Button::new("Retour").on_click(|_, data: &mut AppState, _| {
        data.view = AppView::Login;
    });

    Flex::column()
        .with_child(username_label)
        .with_child(username_input.padding(5.0))
        .with_child(password_label)
        .with_child(password_input.padding(5.0))
        .with_child(repeat_password_label)
        .with_child(repeat_password_input.padding(5.0))
        .with_child(register_button.padding(10.0))
        .with_child(back_button.padding(10.0))
        .with_child(status_label.padding(10.0))
}

// Construire l'interface principale avec navigation
fn build_ui() -> impl Widget<AppState> {
    // Basculer entre les vues basées sur l'État
    druid::widget::Either::new(
        |data, _| data.view == AppView::Login,
        build_login_ui(),
        druid::widget::Either::new(
            |data, _| data.view == AppView::Register,
            build_register_ui(),
            build_connected_ui(), // Vue connectée
        ),
    )
}

fn build_connected_ui() -> impl Widget<AppState> {
    Label::new("You are connected") // Un simple label
}

pub fn call() {
    let main_window = WindowDesc::new(build_ui())
        .title("Application")
        .window_size((400.0, 300.0));

    let initial_state = AppState {
        view: AppView::Login,
        login_state: LoginState {
            username: "".to_string(),
            password: "".to_string(),
            status: "Veuillez entrer vos identifiants.".to_string(),
        },
        register_state: RegisterState {
            username: "".to_string(),
            password: "".to_string(),
            repeat_password: "".to_string(),
            status: "".to_string(),
        },
    };

    // Utiliser AppLauncher avec le délégué pour gérer les commandes
    AppLauncher::with_window(main_window)
        .delegate(Delegate) // Associer le délégué
        .use_simple_logger()
        .launch(initial_state)
        .expect("Erreur au lancement de l'application");
}
