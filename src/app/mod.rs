use getset::Getters;

#[derive(Clone,Getters)]
pub struct App {
    #[getset(get = "pub")]
    extensions: Vec<&'static str>,

    #[getset(get = "pub")]
    jpeg_quality: u8,
}

impl App {
    pub fn new() -> Self {
        let extensions = vec![
            "jpg", "jpeg", "png", "gif", "bmp",
        ];

        Self {
            extensions: extensions,
            jpeg_quality: 80,
        }
    }

}
