use std::fs;
// use xilem::{button, h_stack, v_stack, Adapt, App, AppLauncher, LayoutObserver, Memoize, View};
use sha2::{Digest, Sha256};
use xilem::*;

// https://askubuntu.com/questions/912545/how-to-retrive-a-single-file-from-github-using-git

fn compute_hash(i: usize) -> String {
    let mut s = format!("{}", i);
    for _ in 0..i {
        let mut hasher = Sha256::new();
        hasher.update(s.as_bytes());
        let result = hasher.finalize();
        s = hex::encode(result);
    }
    s
}

#[derive(Default)]
struct AppData {
    count: u32,
    doubleyous: String,
    paths: Vec<String>,
}
impl AppData {
    fn new() -> AppData {
        let paths = fs::read_dir("/home/max/projects/").unwrap();

        let mut app_data = AppData::default();

        for path in paths {
            app_data
                .paths
                .push(path.unwrap().path().display().to_string());
        }
        app_data
    }
}

fn count_button(count: u32) -> impl View<u32> {
    button(format!("count: {}", count), |data| *data += 1)
}

fn app_logic(data: &mut AppData) -> impl View<AppData> {
    scroll_view(async_list(10_000, 16.0, |i| async move {
        format!("{}: {}", i, compute_hash(i))
    }))
    // v_stack((
    //     "fuck".to_string(),
    //     list(3, 50., |i| format!("{}: {}", i, compute_hash(i))),
    // ))
    // list(10, 50., |i| {
    //     let path = data.paths[i].clone();
    //     v_stack((
    //         format!("{}: {}", i, compute_hash(i)),
    //         // format!("{}: {}", i, compute_hash(i)),
    //         format!("{}: {}", i, path),
    //     ))
    // })
    // h_stack((
    //     // scroll_view(async_list(10_000, 16.0, |i| async move {
    //     //     format!("{}: {}", i, compute_hash(i))
    //     // })),
    //     list(3, 50., |i| format!("{}: {}", i, compute_hash(i))),
    //     format!("text: {}", data.doubleyous),
    //     format!("count: {}", data.count),
    //     v_stack((
    //         button("reset", |data: &mut AppData| data.count = 0),
    //         button("add", |data: &mut AppData| data.count += 1),
    //         button("w", |data: &mut AppData| data.doubleyous.push('w')),
    //     )), // Memoize::new(data.count, |count| {
    //         //     button(format!("count: {}", count), |data: &mut AppData| {
    //         //         data.count += 1
    //         //     })
    //         // }),
    //         // Adapt::new(
    //         //     |data: &mut AppData, thunk| thunk.call(&mut data.count),
    //         //     count_button(data.count),
    //         // ),
    //         // LayoutObserver::new(|size| format!("size: {:?}", size)),
    // ))
}

pub fn main() {
    let app = App::new(AppData::new(), app_logic);
    AppLauncher::new(app).run();
}
