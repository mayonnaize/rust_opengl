use std::collections::HashMap;
use std::os::raw::c_void;
use std::path::Path;

use image::GenericImageView;

pub struct ImageManager {
    image_map: HashMap<String, u32>,
}

impl ImageManager {
    pub fn new() -> ImageManager {
        let image_manager = ImageManager {
            image_map: HashMap::new(),
        };
        image_manager
    }

    // 画像の読み込みとGPUへのテクスチャーの登録
    pub fn load_image(&mut self, path: &Path, id: &str, vflip: bool) -> bool {
        if !path.exists() {
            return false;
        }

        let mut image = image::open(path).expect("failed to load image");
        // 読み込んだファイルのフォーマットを設定
        let format = match image {
            // よくわからんけどこんだけの種類があるらしい
            image::ImageLuma8(_) => gl::RED,
            image::ImageLumaA8(_) => gl::RG,
            image::ImageRgb8(_) => gl::RGB,
            image::ImageRgba8(_) => gl::RGBA,
            image::ImageBgr8(_) => gl::RGB,
            image::ImageBgra8(_) => gl::RGBA,
        };
        if vflip {
            image = image.flipv();
        }

        let data = image.raw_pixels();

        let mut texture = 0;

        unsafe {
            // 画像フォーマット アドレスを入れている？
            gl::GenTextures(1, &mut texture);
            // テクスチャーのIDの紐づけ
            gl::BindTexture(gl::TEXTURE_2D, texture);
            // テクスチャーの設定
            /*
                * TEXTURE_WRAP_S (_T)
                    なんかこの辺の設定が出来る
                    * REPEAT
                    * CLAMP_TO_EDGE
                    * MIRRORED_REPEAT

                * TEXTURE_MIN_FILTER
                    テクスチャーを縮小する際の一つのピクセルに置ける色の決定方法
                * TEXTURE_MAG_FILTER
                    テクスチャーを拡大する際の一つのピクセルに置ける色の決定方法
                    この辺が設定できる
                    * LINEAR
                    * NEAREST
             */
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S /* 横軸方向 */, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T /* 縦軸方向 */, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            // 多分基本的には上の四つ(他にもある？)の設定が以下のメソッド実行時のルールとして適用される感じかな
            gl::TexImage2D(
                // テクスチャーのターゲット
                gl::TEXTURE_2D,
                // ミップマップ 極端な縮小すると画像がつぶれる
                // これはなぜ0を入れている？
                0,
                // テクスチャーの内部での色のフォーマット
                format as i32,
                // 画像の幅
                image.width() as i32,
                // 画像の高さ
                image.height() as i32,
                // ボーダーの幅
                0,
                // テクスチャーのピクセルのフォーマット
                format,
                // ピクセルのフォーマット
                gl::UNSIGNED_BYTE,
                // 転送するデータ
                &data[0] as *const u8 as *const c_void,
            );
            // ミップマップの作成
            gl::GenerateMipmap(gl::TEXTURE_2D);
            // 紐づけの解除かな 別のアドレスを入れているっぽい
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        self.image_map.insert(id.to_string(), texture);

        true
    }

    pub fn get_texture_id(&mut self, id: &str) -> u32 {
        *self.image_map.get(id).expect("failed to get texture")
    }
}
