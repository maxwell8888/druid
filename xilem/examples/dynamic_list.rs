use xilem::{
    button, h_stack, v_stack, Adapt, App, AppLauncher, LayoutObserver, Memoize, Top, View, Leading,
};

#[derive(Default)]
struct AppData {
    count: u32,
    doubleyous: String,
}

fn count_button(count: u32) -> impl View<u32> {
    button(format!("count: {}", count), |data| *data += 1)
}

fn app_logic(data: &mut AppData) -> impl View<AppData> {
    h_stack((
        format!("text: {}", data.doubleyous),
        format!("count: {}", data.count),
        v_stack((
            button("reset", |data: &mut AppData| data.count = 0),
            button("add", |data: &mut AppData| data.count += 1),
            button("w", |data: &mut AppData| data.doubleyous.push('w')),
        ))
        .cross_axis_alignment(&Leading), // Memoize::new(data.count, |count| {
                              //     button(format!("count: {}", count), |data: &mut AppData| {
                              //         data.count += 1
                              //     })
                              // }),
                              // Adapt::new(
                              //     |data: &mut AppData, thunk| thunk.call(&mut data.count),
                              //     count_button(data.count),
                              // ),
                              // LayoutObserver::new(|size| format!("size: {:?}", size)),
    ))
    .cross_axis_alignment(&Top)
}

pub fn main() {
    let app = App::new(AppData::default(), app_logic);
    AppLauncher::new(app).run();
}
