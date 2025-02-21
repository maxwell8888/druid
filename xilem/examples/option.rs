use xilem::{
    button, column, h_stack, v_stack, Adapt, App, AppLauncher, LayoutObserver, Memoize, View,
};

#[derive(Default)]
struct AppData {
    count: u32,
    optional_text: Option<String>,
    my_list: Vec<String>,
}

fn count_button(count: u32) -> impl View<u32> {
    button(format!("count: {}", count), |data| *data += 1)
}

fn app_logic(data: &mut AppData) -> impl View<AppData> {
    v_stack((
        column(data.my_list.clone()),
        format!("count: {}", data.count),
        data.optional_text.clone(),
        button("reset", |data: &mut AppData| data.count = 0),
        button("add", |data: &mut AppData| data.count += 1),
        button("add num to text", |data: &mut AppData| {
            data.my_list.push(data.count.to_string())
        }),
        button("toggle text", |data: &mut AppData| {
            data.optional_text = if data.optional_text.is_some() {
                None
            } else {
                Some(data.count.to_string())
            };
            dbg!(&data.optional_text);
        }),
    ))
}

pub fn main() {
    let app = App::new(
        AppData {
            count: 0,
            optional_text: Some("hi\nfriend".to_string()),
            my_list: vec![
                "one".to_string(),
                "two".to_string(),
                "three".to_string(),
                "four".to_string(),
            ],
        },
        app_logic,
    );
    AppLauncher::new(app).run();
}
