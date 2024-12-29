use std::mem;
use std::os::raw::c_void;
use std::time::Duration;

use c_str_macro::c_str;
use cgmath::perspective;
use cgmath::prelude::SquareMatrix;
use gl::types::{GLfloat, GLsizei, GLsizeiptr};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

// 多分インクルード
mod shader;
mod vertex;

use shader::Shader;
use vertex::Vertex;

#[allow(dead_code)]
type Point3 = cgmath::Point3<f32>;
#[allow(dead_code)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;
const FLOAT_NUM: usize = 3;
const VERTEX_NUM: usize = 3;
// const VERTEX_NUM: usize = 4;
const BUF_LEN: usize = FLOAT_NUM * VERTEX_NUM;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    {   // SDLに対応したOpenGLを取得 3.1を指定
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 1);
        let (major, minor) = gl_attr.context_version();
        println!("OK: init OpenGL: version={}.{}", major, minor);
    }

    let window = video_subsystem
        .window("SDL", WINDOW_WIDTH, WINDOW_HEIGHT)
        .opengl()                   // OpenGLを有効化
        .position_centered()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    /* gl::load_withの引数がラムダ式を受け取る
        ↓ 引数の中身はC++だとこうなるっぽい
        [](const char* name) {
            void* p = SDL_GL_GetProcAddress(name);
            return p;
        };
        as _ は戻り値の型を省略

        OpenGL関数ポインタをロードするために必要です。
        これにより、OpenGLの関数をRustのコード内で使用できるようになります。
        SDL2のgl_get_proc_addressを使用して、OpenGL関数のアドレスを取得し、それをglクレートに登録します。
        これがないと、OpenGL関数を呼び出すことができません。

        こんなもん別に内部でやればいいように思うが新規メソッドとか追加されたときとかに役立つのだろうか
        基本的にはおまじないの認識で良さげ
     */
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _);

    let shader = Shader::new("rsc/shader/shader.vs", "rsc/shader/shader.fs");

    // set buffer
    #[rustfmt::skip]
    let buffer_array: [f32; BUF_LEN] = [
        // 三角形
        -1.0, -1.0, 0.0,        // 左下
        1.0, -1.0, 0.0,         // 右下
        0.0, 1.0, 0.0,          // 右上
        // 四角形
        // -1.0, 1.0, 0.0,         // 左上
    ];

    // バーテックスシェーダー: 線を描画
    // フラグメントシェーダー: 色を描画
    // バーテックス=頂点
    let vertex = Vertex::new(
        // * 頂点データのデータサイズ
        (BUF_LEN * mem::size_of::<GLfloat>()) as GLsizeiptr,
        // * 頂点データのポインタ
        buffer_array.as_ptr() as *const c_void,
        // * どのようなアクセス頻度でデータを扱うことになるのかを示す値
        gl::STATIC_DRAW,
        // * 各頂点属性のデータ型を格納したベクター型
        vec![gl::FLOAT],
        // * 各頂点属性のデータサイズを格納したベクター型
        vec![FLOAT_NUM as i32],
        // * 各頂点データの始まりが何個おきに並んでいるのか
        FLOAT_NUM as i32 * mem::size_of::<GLfloat>() as GLsizei,
        // * 頂点の数
        VERTEX_NUM as i32,
    );

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        unsafe {
            // ビューポートの設定
            gl::Viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);

            // clear screen
            gl::ClearColor(1.0, 1.0, 0.0, 1.0); // RGBA
            // DEPTH_BUFFER_BIT, STENCIL_BUFFER_BITがある
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // init matrice for model, view and projection
            // * モデル行列
            // * モデル行列とは、描画したい物体を平行移動、回転、拡大、縮小を行う際に使う行列です。
            // * 何もしない場合は、単位行列(対角成分だけが1で、残りは0)を指定しましよう。
            let model_matrix = Matrix4::identity();
            // * ビュー行列
            let view_matrix = Matrix4::look_at(
                // 観測者の位置
                Point3 {
                    x: 0.0,
                    y: 0.0,
                    z: 5.0,
                },
                // 見ている物の位置
                Point3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                // どちらを上にするか
                Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
            );
            // * 射影行列
            // 平行投影(遠近感なし)・透視投影(遠近感あり)
            // perspectiveで透視投影になる
            // orthoで平行投影になる
            let projection_matrix: Matrix4 = perspective(
                cgmath::Deg(45.0f32),
                WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32,
                0.1,
                100.0,
            );

            // shader use matrices
            shader.use_program();
            // モデル行列の適用
            shader.set_mat4(c_str!("uModel"), &model_matrix);
            // ビュー行列の適用
            shader.set_mat4(c_str!("uView"), &view_matrix);
            // 射影行列の適用
            shader.set_mat4(c_str!("uProjection"), &projection_matrix);

            vertex.draw();
            // 描画内容を更新
            window.gl_swap_window();
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
