use ddp_rs::{connection, protocol};
use engine::display::Display;
use mario::clockface::Clockface;
const GRID_SIZE: usize = 64;

mod engine;
mod mario;

pub trait ClockfaceTrait {
    fn update(&mut self, display: &mut Display);
    fn setup(&mut self, display: &mut Display);
}

#[tokio::main]
async fn main() -> ! {
    let mut cf = Clockface::new();

    let mut display = Display::new();

    cf.setup(&mut display);

    let mut conn = connection::DDPConnection::try_new(
        "192.168.178.63:4048",
        protocol::PixelConfig::default(),
        protocol::ID::Default,
        std::net::UdpSocket::bind("0.0.0.0:4048").unwrap(),
    )
    .expect("Failed to create connection");

    loop {
        cf.update(&mut display);

        let buffer = display.get_buffer();
        let u32_buffer: Vec<u8> = buffer
            .iter()
            .flat_map(|&color| {
                let r = ((color >> 11) & 0x1F) as u8;
                let g = ((color >> 5) & 0x3F) as u8;
                let b = (color & 0x1F) as u8;
                vec![r, g, b]
            })
            .collect();

        conn.write(u32_buffer.as_slice())
            .expect("Failed to write buffer");

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
