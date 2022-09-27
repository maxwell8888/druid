use xilem::{
    button, h_stack, list, v_stack, Adapt, AnyView, App, AppLauncher, LayoutObserver, Memoize, View,
};

struct Item {
    id: usize,
    name: String,
    contents: String,
}

#[derive(Default)]
struct AppData {
    count: u32,
    doubleyous: String,
    items: Vec<Item>,
    page: usize,
}

fn count_button(count: u32) -> impl View<u32> {
    button(format!("count: {}", count), |data| *data += 1)
}

fn optional_content(data: &mut AppData) -> Box<dyn AnyView<AppData> + Send + 'static> {
    if data.count > 5 {
        return Box::new(format!("biiig count: {}", data.count));
    } else {
        let h = h_stack((
            format!("count: {}", data.count),
            button("reset", |data: &mut AppData| data.count = 0),
        ));
        return Box::new(h);
    };
}

// fn app_logic(data: &mut AppData) -> impl View<AppData> {
fn app_logic(data: &mut AppData) -> Box<dyn AnyView<AppData> + Send + 'static> {
    let all = h_stack((
        format!("text: {}", data.doubleyous),
        format!("count: {}", data.count),
        // list(10, 30., |i| format!("name {}: {}", i, data.items[i].name)),
        list(10, 30., |i| format!("hello: {}", i)),
        optional_content(data),
        if data.count > 5 {
            Box::new(format!("biiig count: {}", data.count))
                as Box<dyn AnyView<AppData> + Send + 'static>
        } else {
            let h = h_stack((
                format!("count: {}", data.count),
                button("reset", |data: &mut AppData| data.count = 0),
            ));
            Box::new(h) as Box<dyn AnyView<AppData> + Send + 'static>
        },
        v_stack((
            button("reset", |data: &mut AppData| data.count = 0),
            button("add", |data: &mut AppData| data.count += 1),
            button("w", |data: &mut AppData| data.doubleyous.push('w')),
        )),
        h_stack((
            button("<", |data: &mut AppData| data.page -= 1),
            button(">", |data: &mut AppData| data.page += 1),
        )),
        {
            let subitems = data
                .items
                .iter()
                .enumerate()
                .filter(|(i, _item)| *i >= data.page * 2 && *i < (data.page + 1) * 2)
                .collect::<Vec<_>>();
            // let page = (
            //     h_stack((
            //         format!("id: {}, name: {}", "fart", "beans"),
            //         format!("arse"),
            //     )),
            //     h_stack((
            //         format!("id: {}, name: {}", "fart", "beans"),
            //         format!("arse"),
            //     )),
            // );
            v_stack((
                format!("id: {}, name: {}", subitems[0].1.id, subitems[0].1.name),
                format!("id: {}, name: {}", subitems[1].1.id, subitems[1].1.name),
            ))
            // v_stack((format!("arse"), format!("arse")))
        },
        // Memoize::new(data.count, |count| {
        //     button(format!("count: {}", count), |data: &mut AppData| {
        //         data.count += 1
        //     })
        // }),
        // Adapt::new(
        //     |data: &mut AppData, thunk| thunk.call(&mut data.count),
        //     count_button(data.count),
        // ),
        // LayoutObserver::new(|size| format!("size: {:?}", size)),
    ));
    Box::new(all)
}
// fn app_logic(data: &mut AppData) -> impl View<AppData> {
//     if data.count > 5 {
//         return format!("biiig count: {}", data.count);
//     } else {
//         return button("reset", |data: &mut AppData| data.count = 0);
//     };
// }

pub fn main() {
    let mut app_data = AppData::default();
    app_data.items = vec![
        Item {
            id: 0,
            name: "Bruce".to_string(),
            contents: "I went to the park".to_string(),
        },
        Item {
            id: 1,
            name: "Cowman".to_string(),
            contents: "I went to the shops".to_string(),
        },
        Item {
            id: 2,
            name: "Alan".to_string(),
            contents: "EAting ice cream is fun".to_string(),
        },
        Item {
            id: 3,
            name: "Tom".to_string(),
            contents: "When is the moon coming?".to_string(),
        },
    ];
    let app = App::new(app_data, app_logic);
    AppLauncher::new(app).run();
}
