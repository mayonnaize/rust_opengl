use std::ptr;

pub struct FrameBuffer {
    frame_buffer: u32,
    render_buffer: u32,
    texture_color_buffer: u32,
}

impl FrameBuffer {
    pub fn new(width: u32, height: u32) -> FrameBuffer {
        let mut frame_buffer: u32 = 0;
        let mut render_buffer: u32 = 0;
        let mut texture_color_buffer: u32 = 0;

        unsafe {
            // FBOの生成および紐づけ
            gl::GenFramebuffers(1, &mut frame_buffer);
            gl::BindFramebuffer(gl::FRAMEBUFFER, frame_buffer);

            // FBOに描画した内容を別のところに貼り付ける際には、このテクスチャを使って描画
            // init a color attachment texture
            gl::GenTextures(1, &mut texture_color_buffer);
            gl::BindTexture(gl::TEXTURE_2D, texture_color_buffer);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                width as i32,
                height as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                ptr::null(),
            );
            // テクスチャをフレームバッファーオブジェクトにアタッチする
            gl::FramebufferTexture2D(
                // フレームバッファーのターゲットを指定
                gl::FRAMEBUFFER,
                // テクスチャをどのバッファーにアタッチするか
                // ! ここではテクスチャをカラーバッファーにアタッチしている
                gl::COLOR_ATTACHMENT0,
                // テクスチャのターゲット
                gl::TEXTURE_2D,
                // テクスチャのID
                texture_color_buffer,
                // ミップマップレベル 必ず0
                0,
            );
            gl::BindTexture(gl::TEXTURE_2D, 0);

            // ! デプスバッファー
            // init render buffer object
            gl::GenRenderbuffers(1, &mut render_buffer);
            gl::BindRenderbuffer(gl::RENDERBUFFER, render_buffer);
            // 実際の保存域の確保
            gl::RenderbufferStorage(
                // メモリ確保するターゲット
                // ! 常にgl::RENDERBUFFER
                gl::RENDERBUFFER,
                // レンダーバッファーのフォーマット
                gl::DEPTH_COMPONENT24,
                // 幅
                width as i32,
                // 高さ
                height as i32,
            );
            gl::FramebufferRenderbuffer(
                gl::FRAMEBUFFER,
                // レンダーバッファーをどのバッファーにアタッチするか
                gl::DEPTH_ATTACHMENT,
                // レンダーバッファーのターゲット
                gl::RENDERBUFFER,
                render_buffer,
            );
            gl::BindRenderbuffer(gl::RENDERBUFFER, 0);

            // ! フレームバッファーの状態の確認
            // check frame buffer status
            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                println!("error: frame buffer is not complete");
            }

            // bind default frame buffer
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        FrameBuffer {
            frame_buffer: frame_buffer,
            render_buffer: render_buffer,
            texture_color_buffer: texture_color_buffer,
        }
    }

    pub fn bind_as_frame_buffer(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.frame_buffer);
        }
    }

    pub fn bind_as_texture(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_color_buffer);
        }
    }
}

// C++でいうところのデストラクタ
// Dropトレイト
impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe {
            // ヨーダ記法？
            if 0 != self.frame_buffer {
                gl::DeleteFramebuffers(1, &self.frame_buffer);
                self.frame_buffer = 0;
            }
            if 0 != self.texture_color_buffer {
                gl::DeleteTextures(1, &self.texture_color_buffer);
                self.texture_color_buffer = 0;
            }
            if 0 != self.render_buffer {
                gl::DeleteRenderbuffers(1, &self.render_buffer);
                self.render_buffer = 0;
            }
        }
    }
}
