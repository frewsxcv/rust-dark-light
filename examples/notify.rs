fn main() -> anyhow::Result<()> {
    dark_light::notify(&change_color_scheme)
}

fn change_color_scheme(mode: dark_light::Mode) {
    println!("Changing to {:?}", mode)
}
