mod utils;
mod structs;
mod reader;

use std::path::Path;
use std::sync::Arc;
use futures_util::StreamExt;
use reqwest;
use reqwest::{blocking, Client};
use serde::{Deserialize, Serialize};
use serde_json;
use slint::{Image, Model, ModelRc, SharedString, VecModel, Weak};
use tokio;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::sync::Mutex;
use futures_util::future::Shared;
use crate::reader::controlla;
use crate::utils::api::Api;
use crate::structs::MapSet;
use crate::structs::Map;
use crate::utils::rate::{change_audio_speed, change_osu_speed};

///
/// SLINT MODULE SI JE LE DELETE ENCORE JE SUIS UNE PUTE
///
slint::include_modules!();
///
///  SLINT MODULE
///

#[derive(Debug, Serialize, Deserialize)]
struct MapSetResponse {
    mapSet: Vec<MapSet>,
}


#[tokio::main(flavor = "multi_thread")]
pub async fn main() -> Result<(), Box<dyn std::error::Error>>
{

    let api = Arc::new(Api::new());
    let login_page = LoginPage::new()?;
    login_page.global::<AppState>().set_is_logged_in(false);
    login_page.global::<AppState>().set_token(SharedString::from(""));
    login_page.global::<AppState>().set_current_page(0);




    let window_handle = login_page.as_weak();
    let api_for_closure = api.clone();

    let pp_window: Arc<Mutex<Option<PPWindow>>> = Arc::new(Mutex::new(None));
    let pp_window_clone = pp_window.clone();

    let pp_window_clone = pp_window.clone();

    if let Ok(window_lock) = pp_window.lock() {
        if let Some(window) = window_lock.as_ref() {
            window.global::<PPSettings>().on_window_updated(move || {});
        }
    }



    login_page.global::<AppState>().on_update_map(move |map_data| {
        if let Some(handle) = window_handle.clone().upgrade() {
            println!("called");
            handle.global::<AppState>().set_map(map_data);
        }
    });
    let window_handle = login_page.as_weak();

    login_page.global::<AppState>().on_change_rate(move |rate| {
        println!("called");
        if let Some(handle) = window_handle.upgrade() {
            println!("called");
            let input_path = handle.global::<AppState>().get_audio_path().to_string();
            let file_path = handle.global::<AppState>().get_osu_path().to_string();
            let input_path = Path::new(&input_path);
            let parent = input_path.parent().unwrap_or(Path::new(""));
            let new_filename = format!("audio_{}.ogg", (rate * 100.0) as i32);
            let output_path = parent.join(new_filename.clone());


            let output_path =output_path.display().to_string();
            let input_path = input_path.display().to_string();

            if let Err(err) = change_audio_speed(&input_path, &output_path, rate) {
                eprintln!("Erreur lors du changement de vitesse audio : {:#}", err);
            }
            change_osu_speed(& file_path, rate, &new_filename);
        }

    });

    let window_handle = login_page.as_weak();

    login_page.global::<AppState>().on_toggle_pp_window(move |checked| {
        if checked {
            let new_window = PPWindow::new().unwrap();
            new_window.show().unwrap();
            *pp_window_clone.lock().unwrap() = Some(new_window);
            if let Some(window) = pp_window_clone.lock().unwrap().as_ref() {
                let window_handle = window.clone_strong();
                window.on_update_text(move |new_text| {
                    window_handle.set_pptext(format!("{} pp", new_text).into());
                });
            }

        } else {
            if let Some(window) = pp_window_clone.lock().unwrap().take() {
                window.hide().unwrap();
            }
        }
    });





    login_page
        .global::<MapSetState>()
        .on_download(move |link, index| {
            let api = api_for_closure.clone();
            let lien = link.to_string();
            let weak = window_handle.clone();
            let path = format!("{}/{}_map.osz",
                               window_handle.clone().upgrade().unwrap().global::<MapSetState>().get_osu_path(),
                               index
            );
            if let Some(window) = pp_window.lock().unwrap().as_ref() {
                window.invoke_update_text("140".into());
            }
            tokio::spawn(async move {
                let weak_clone = weak.clone();
                let progress_callback = move |progress: f32| {
                    let weak = weak_clone.clone();
                    slint::invoke_from_event_loop(move || {
                        if let Some(handle) = weak.upgrade() {
                            let current_model = handle.global::<MapSetState>().get_maps();
                            if let Some(model) =
                                current_model.as_any().downcast_ref::<VecModel<MapData>>()
                            {
                                if let Some(mut item) = model.row_data(index as usize) {
                                    item.download_progress = progress;
                                    model.set_row_data(index as usize, item);
                                }
                            }
                        }
                    })
                    .ok();
                };
                match api.download_files(&lien, &path, progress_callback).await {
                    Ok(_) => {
                        if let Some(handle) = weak.upgrade() {
                            let current_model = handle.global::<MapSetState>().get_maps();
                            if let Some(model) =
                                current_model.as_any().downcast_ref::<VecModel<MapData>>()
                            {
                                if let Some(mut item) = model.row_data(index as usize) {
                                    item.download_progress = 1.0;
                                    model.set_row_data(index as usize, item);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Erreur lors du téléchargement: {}", e);
                    }
                }
            });
        });
    let api_for_closure = api.clone();
    login_page.on_load_more({
        let weak = login_page.as_weak().clone();

        move || {
            let api = api_for_closure.clone();

            if let Some(window) = weak.upgrade() {
                let current_page = window.get_current_page();
                let weak_clone = weak.clone();

                slint::spawn_local(async move {
                    if let Err(e) = get_maps(api,current_page as usize, &weak_clone).await {
                        eprintln!("Erreur lors du chargement des maps: {}", e);
                        return;
                    }

                    if let Some(window) = weak_clone.upgrade() {
                        window.set_current_page(current_page + 10);
                    }
                });
            }
        }
    });



    let api_for_closure = api.clone();

    login_page.on_login_requested({
        let weak = login_page.as_weak();

        move || {
            let api = api_for_closure.clone();

            let username = weak.clone().unwrap().get_ausername().to_string();
            let password = weak.clone().unwrap().get_apassword().to_string();
            let weak = weak.clone();

            slint::spawn_local(async move {
                match api.check_credentials(&username, &password).await {
                    Ok(token) => {
                        if let Some(window) = weak.upgrade() {
                            window.global::<AppState>().set_is_logged_in(true);
                        }
                    }
                    Err(e) => {
                        eprintln!("Échec de la connexion: {}", e);
                    }
                }
            });
        }
    });
    let weak = login_page.as_weak();

    tokio::spawn(async move {
        controlla(weak);
    });
    login_page.show();
    slint::run_event_loop();
    Ok(())
}

async fn get_maps(api: Arc<Api>, page: usize, weak: &Weak<LoginPage>) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(response) = api.fetch_mapsets(page as i32).await {
        for map_set in response.mapSet{
            let difficulties = map_set
                .maps
                .iter()
                .map(|m| format!("{:.2} ({})", m.difficulty, m.pattern))
                .collect::<Vec<_>>()
                .join(", ");

            let image_data = match tokio::time::timeout(
                std::time::Duration::from_secs(5),
                api.fetch_image(&map_set.cover)
            ).await {
                Ok(Ok(image_bytes)) => process_image(&image_bytes),
                Ok(Err(e)) => {
                    eprintln!("Erreur lors du téléchargement de l'image: {}", e);
                    None
                },
                Err(_) => {
                    eprintln!("Timeout lors du téléchargement de l'image");
                    None
                }
            };

            let weak_clone = weak.clone();
            update_ui(weak_clone, map_set, difficulties, image_data)?;
        }
    }
    Ok(())
}

fn process_image(image_bytes: &[u8]) -> Option<(u32, u32, Vec<u8>)> {
    match image::load_from_memory(image_bytes) {
        Ok(img) => {
            let rgba = img.into_rgba8();
            Some((rgba.width(), rgba.height(), rgba.into_raw()))
        },
        Err(e) => {
            eprintln!("Erreur lors du chargement de l'image: {}", e);
            None
        }
    }
}

fn update_ui(
    weak: Weak<LoginPage>,
    map_set: MapSet,
    difficulties: String,
    image_data: Option<(u32, u32, Vec<u8>)>
) -> Result<(), Box<dyn std::error::Error>> {
    slint::spawn_local(async move {
        if let Some(window) = weak.upgrade() {
            let map_data = MapData {
                song: SharedString::from(&map_set.song),
                author: SharedString::from(&map_set.author),
                creator: SharedString::from(&map_set.creator),
                cover: if let Some((width, height, raw_data)) = image_data {
                    let mut buffer = slint::SharedPixelBuffer::new(width, height);
                    buffer.make_mut_bytes().copy_from_slice(&raw_data);
                    Image::from_rgba8(buffer)
                } else {
                    Image::default()
                },
                link: SharedString::from(&map_set.link),
                difficulties: SharedString::from(difficulties),
                md5 : SharedString::from(String::new()),
                download_progress: 0.0,
            };

            let current_model = window.global::<MapSetState>().get_maps();
            let mut maps = current_model
                .as_any()
                .downcast_ref::<VecModel<MapData>>()
                .map(|m| m.iter().collect::<Vec<_>>())
                .unwrap_or_default();
            maps.push(map_data);

            window.global::<MapSetState>().set_maps(ModelRc::new(VecModel::from(maps)));
        }
    });
    Ok(())

}





