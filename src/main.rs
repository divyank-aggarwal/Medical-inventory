#![windows_subsystem = "windows"]
use fltk::{
    group::{Group, Pack, Tabs},
    prelude::*,
    *,
};
mod utils;
use chrono;
use fltk_table::{SmartTable, TableOpts};

use utils::{get_initial_values, get_test_values, insert_new_category, Category, History};

use crate::utils::add_value;

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = window::Window::default()
        .with_size(500, 450)
        .with_label("Inventory")
        .center_screen();
    let mut tabs = Tabs::default().with_size(480, 430).with_pos(10, 10);
    let mut grp1 = Group::default()
        .with_size(480, 405)
        .with_label("Inventory")
        .with_pos(10, 35);
    let mut pack1 = Pack::default().with_size(300, 405).with_pos(135, 45);
    pack1.set_spacing(10);
    let mut choice = menu::Choice::default()
        .with_size(0, 30)
        .with_label("Select category");
    let mut values = get_initial_values();
    {
        let values_read = values.read().unwrap();
        for value in values_read.as_slice() {
            choice.add_choice(value.name.as_str());
        }
    }

    let mut name_label = frame::Frame::default()
        .with_size(0, 30)
        .with_label("Category name: ");

    let mut value_label = frame::Frame::default()
        .with_size(0, 30)
        .with_label("Total: ");

    let mut increase_value = input::IntInput::default()
        .with_size(0, 30)
        .with_label("Enter change");

    let mut increase_button = button::Button::default()
        .with_label("Submit")
        .with_size(0, 30);

    let mut add_category_name = input::Input::default()
        .with_label("Enter name")
        .with_size(0, 30);

    let mut add_category_value = input::IntInput::default()
        .with_size(160, 30)
        .with_label("Enter initial value");

    let mut new_cat_button = button::Button::default()
        .with_size(0, 30)
        .with_label("Submit");

    pack1.end();
    grp1.end();
    let grp2 = Group::default()
        .with_size(470, 425)
        .with_label("History")
        .with_pos(10, 35);

    let mut pack2 = Pack::default().with_size(400, 405).with_pos(75, 45);
    pack2.set_spacing(10);
    let mut choice2 = menu::Choice::default()
        .with_size(0, 30)
        .with_label("Category");
    {
        let values_read = values.read().unwrap();
        for value in values_read.as_slice() {
            choice2.add_choice(value.name.as_str());
        }
    }

    let mut table = SmartTable::default().with_size(0, 350);
    table.set_opts(TableOpts {
        rows: 30,
        cols: 3,
        editable: false,
        ..Default::default()
    });
    table.set_col_width(0, 55);
    table.set_col_width(1, 135);
    table.set_col_width(2, 100);
    table.set_col_header_value(0, "+/-");
    table.set_col_header_value(1, "Date");
    table.set_col_header_value(2, "Time");
    grp2.end();
    tabs.end();

    wind.end();
    wind.show();

    choice2.set_callback({
        let values = values.clone();
        let mut table = table.clone();
        move |c| {
            let ind: usize = c.value().try_into().unwrap();
            let values = &values.read().unwrap()[ind];
            table.set_rows(values.history.len() as i32);
            for (i, val) in values.history.iter().enumerate() {
                table.set_cell_value(i as i32, 0, val.value.to_string().as_str());
                table.set_cell_value(i as i32, 1, val.date.to_string().as_str());
                table.set_cell_value(i as i32, 2, &val.time.to_string().as_str()[0..5]);
                table.redraw();
            }
        }
    });

    new_cat_button.set_callback({
        let mut add_cat_name = add_category_name.clone();
        let mut add_cat_value = add_category_value.clone();
        let mut values = values.clone();
        let mut choices = choice.clone();
        let mut choices2 = choice2.clone();
        move |_| {
            let cat_name = add_cat_name.value();
            if cat_name == "" {
                return ();
            };
            let cat_value = add_cat_value.value().parse::<i32>().unwrap_or(0);
            let mut values = values.write().unwrap();
            let date_time = chrono::Local::now();
            let date = date_time.naive_local().date();
            let time = date_time.time();
            insert_new_category(cat_name.as_str(), cat_value);
            values.push(Category {
                name: cat_name.clone(),
                total: cat_value,
                history: vec![History {
                    time,
                    date,
                    value: cat_value as i32,
                }],
            });
            choices.add_choice(&cat_name);
            choice2.add_choice(&cat_name);

            add_cat_name.set_value("");
            add_cat_value.set_value("");
        }
    });

    increase_button.set_callback({
        let mut value_label = value_label.clone();
        let mut choice = choice.clone();
        let mut inc_value = increase_value.clone();
        let values = values.clone();
        move |i| {
            let ind: usize = choice.value().try_into().unwrap_or(0);
            println!("Trying to acquire lock");
            let mut values = values.write().unwrap();
            println!("Acquired write lock");
            let inc = inc_value.value().parse::<i32>().unwrap_or(0);
            values[ind].total = values[ind].total + inc;
            let date_time = chrono::Local::now();
            let date = date_time.naive_local().date();
            let time = date_time.time();
            add_value(&values[ind].name, inc);
            values[ind].history.push(utils::History {
                value: inc,
                date,
                time,
            });
            value_label.set_label(format!("{}", values[ind].total).as_str());
            inc_value.set_value("");
        }
    });

    let values_2 = values.clone();
    choice.set_callback(move |c| {
        let mut name_label = name_label.clone();
        let mut value_label = value_label.clone();
        let ind: usize = c.value().try_into().unwrap();
        let values = values_2.clone();
        name_label.set_label(format!("{}", values.read().unwrap()[ind].name).as_str());
        value_label.set_label(format!("{}", values.read().unwrap()[ind].total).as_str());
    });

    app.run().unwrap();
}
