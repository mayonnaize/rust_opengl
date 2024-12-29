use std::mem;
use std::os::raw::c_void;

use gl::types::{GLenum, GLfloat, GLint, GLsizei, GLsizeiptr};

pub struct Vertex {
    vao: u32,
    _vbo: u32,
    vertex_num: i32,
}

impl Vertex {
    // main.rsで呼び出される
    pub fn new(
        // 頂点データのデータサイズ
        size: GLsizeiptr,
        // 頂点データのポインタ
        data: *const c_void,
        // どのようなアクセス頻度でデータを扱うことになるのかを示す値
        usage: GLenum,
        // 各頂点属性のデータ型を格納したベクター型
        attribute_type_vec: std::vec::Vec<GLenum>,
        // 各頂点属性のデータサイズを格納したベクター型
        attribute_size_vec: std::vec::Vec<GLint>,
        // 各頂点データの始まりが何個おきに並んでいるのか
        stride: GLsizei,
        // 頂点の数
        vertex_num: i32,
    ) -> Vertex {
        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            /*
                古いやり方だと↓になるのをいい感じにしている
                glBegin(GL_TRIANGLES);
                glVertex2f(-1.0, -1.0);
                glVertex2f(1.0, -1.0);
                glVertex2f(0.0, 1.0);
                glEnd();

                VBO: バーテックスバッファーオブジェクト
                    * GPU側のメモリ上に頂点データを置いておくための領域です。
                    * GPUで描画するためには、CPU側からGPU側へデータを送る必要があるため
                    * バーテックスバッファーオブジェクトを作って、その中に頂点データを保存しておく
                VAO: バーテックスアレイオブジェクト
                    * 頂点データをどのようなまとまりで使うのかを設定するものです。
                    * 頂点データは、ただのfloat型の配列でしかありません。
                    * たとえば、その配列を3個のfloat型ごとに3次元の座標として扱い、ひとまとめにすることができます。
                    * 少し複雑な例では、3次元の座標の他に、法線ベクトルと色情報もあわせた、10個のfloat型でひとまとめにすることもあります。
             */
            // GPUメモリ上に確保
            // 複数のVAO, VBOを確保できるが一つだけの為1を第一引数にしている
            // create vertex array object and vertex buffer object
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            // bind buffer
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            // VBOに初めてデータを転送する時に使用する
            // usageの取りうる値
            // * STATIC_DRAW  低 高 データをあまり変更せず、それを頻繁に利用する場合に指定します。
            // * DYNAMIC_DRAW  高 高 データを何度か変更し、それを頻繁に利用する場合に指定します。
            // * STREAM_DRAW  低 低 データをあまり変更せず、それを数回利用する場合に指定します。
            gl::BufferData(gl::ARRAY_BUFFER, size, data, usage);
            //  →二回目以降はgl::BufferSubData()を使用

            let mut offset = 0;
            for i in 0..attribute_type_vec.len() {
                // 頂点属性の配列の有効化
                gl::EnableVertexAttribArray(i as u32);
                // これから GPUへ送る頂点属性のデータが、どのようなまとまりになっているのか設定します。
                gl::VertexAttribPointer(
                    // 頂点属性の順番(0から始まります)
                    i as u32,
                    // 頂点属性あたりの要素数
                    attribute_size_vec[i],
                    // データ型
                    attribute_type_vec[i],
                    // 整数型を浮動小数点型に正規化するか否かを示す boolean
                    gl::FALSE,
                    // 各頂点データの始まりが何個おきに並んでいるのか
                    stride,
                    // 頂点データの開始地点のオフセットを与えます。
                    (offset * mem::size_of::<GLfloat>()) as *const c_void,
                );
                offset += attribute_size_vec[i] as usize;
            }

            // unbind
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Vertex {
            vao: vao,
            _vbo: vbo,
            vertex_num: vertex_num,
        }
    }

    // 座標データをGPUに送り描画する
    pub fn draw(&self) {
        unsafe {
            // デフォルトは三角形
            let mut shape: GLenum = gl::TRIANGLES;
            if self.vertex_num == 4 {
                // 四角形
                shape = gl::QUADS;
            }
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(
                // 描画するプリミティブなるものの種類の指定
                shape,
                // 頂点データの開始インデックス
                0,
                // 描画する頂点の数を引数
                self.vertex_num
            );
            gl::BindVertexArray(0);
        }
    }
}
