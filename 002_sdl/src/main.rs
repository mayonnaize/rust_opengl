use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

fn main() {
    // SDL2の初期化 SDL構造体を取得
    let sdl_context = sdl2::init().unwrap();
    // ウィンドウの初期化 ウィンドウ構造体を取得
    let video_subsystem = sdl_context.video().unwrap();

    // let window = video_subsystem
    //     .window("SDL", 640, 480)
    //     .position_centered()
    //     // .fullscreen_desktop()
    //     .build()
    //     .unwrap();   // Result型で返ってくるものを成功する前提で使う

    let window = match video_subsystem
        .window("SDL", 640, 840)
        .position_centered()
        .build()
        {
            Ok(window) => window,
            Err(err) => panic!("failed to build window: {:?}", err),
        };

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();       // 描画

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {                                    // ラベル付きループ
        for event in event_pump.poll_iter() {    // イベントキューにたまっているイベントを処理
            match event {
                Event::Quit { .. }                      // ×ボタン
                | Event::KeyDown {                      // Escキー
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}                                 // 上記以外のケースを無視する defaultみたいなやつ
            }
        }
        // あれば他の描画処理を行う
        canvas.present();       // 再描画

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));  // 60FPS
    }
}
