extern crate opencv;
use opencv::highgui;
use opencv::imgcodecs;
use std::io::{Error};

use opencv::videoio::{VideoCapture, CAP_FFMPEG, CAP_PROP_FPS, CAP_PROP_FRAME_COUNT, CAP_PROP_FRAME_HEIGHT, CAP_PROP_FRAME_WIDTH, CAP_PROP_POS_FRAMES};
use opencv::videoio::prelude::VideoCaptureTrait;
use opencv::dnn::prelude::NetTrait;
use opencv::dnn;
use opencv::core::{Mat, Vector, Scalar, Size, CV_32F};
use opencv::types::{VectorOfString};
use opencv::imgproc::{cvt_color, COLOR_RGB2RGBA};
use opencv::core::prelude::MatTrait;
// use opencv::objdetect::{CascadeClassifier};
// use opencv::objdetect::CascadeClassifierTrait;
use opencv::core::{Rect};
use opencv::{imgproc, objdetect, prelude::*, types};

const WINDOW_NAME: &str = "Optimus Engine Visual Debugger";

https://www.pyimagesearch.com/2019/09/02/opencv-stream-video-to-web-browser-html-page/

const CAPTURE_WIDTH: i32 = 800;
const CAPTURE_HEIGHT: i32 = 600;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_video_file = "/home/cosmotek/Downloads/The.Unicorn.S01E12.HDTV.x264-KILLERS[ettv]/The.Unicorn.S01E12.HDTV.x264-KILLERS[ettv].mkv";
    let classifer_file = "/home/cosmotek/code/rust/homesec/haarcascade_frontalface_default.xml";

    let mut vid = VideoCapture::from_file(input_video_file, CAP_FFMPEG).unwrap();
    vid.set(CAP_PROP_POS_FRAMES, 4000.0);
    let mut class = objdetect::CascadeClassifier::new(classifer_file)?;

    highgui::named_window(WINDOW_NAME, 0).unwrap();
    highgui::resize_window(WINDOW_NAME, 800, 600).unwrap();

    loop {
        let mut oldframe = Mat::default()?;
        let next_exists = vid.read(&mut oldframe).unwrap();
        // let mut frame = Mat::default()?;
        // imgproc::cvt_color(&mut oldframe, &mut frame, imgproc::COLOR_BGR2GRAY, 0)?;

        if next_exists {
            const SCALE_FACTOR: f64 = 1.2;
            const SCALE_FACTOR_INV: i32 = (1f64 / SCALE_FACTOR) as i32;

            const MIN_NEIGHBORS: i32 = 2;
            const FLAGS: i32 = 0;
            const MIN_FACE_SIZE: Size = Size {
                width: 10,
                height: 10,
            };
            const MAX_FACE_SIZE: Size = Size {
                width: 250,
                height: 250,
            };

            let mut faces = types::VectorOfRect::new();
            class.detect_multi_scale(
                &oldframe,
                &mut faces,
                SCALE_FACTOR,
                MIN_NEIGHBORS,
                FLAGS,
                MIN_FACE_SIZE,
                MAX_FACE_SIZE,
            )?;

            // println!("{} faces", faces.len());
            for face in faces {
                // println!("{}x{} [{}, {}]", face.x, face.y, face.width, face.height);
                let scaled_face = Rect {
                    x: (face.x - 100) * SCALE_FACTOR_INV,
                    y: (face.y - 100) * SCALE_FACTOR_INV,
                    width: face.width * SCALE_FACTOR_INV,
                    height: face.height * SCALE_FACTOR_INV,
                };

                const THICKNESS: i32 = 2;
                const LINE_TYPE: i32 = 8;
                const SHIFT: i32 = 0;
                let color_red = Scalar::new(0f64, 0f64, 255f64, -1f64);
            
                // imgproc::cvt_color(&mut frame, &mut oldframe, imgproc::COLOR_GRAY2BGR, 0)?;
                imgproc::rectangle(&mut oldframe, scaled_face, color_red, THICKNESS, LINE_TYPE, SHIFT)?;
                imgproc::rectangle(&mut oldframe, face, Scalar::new(0f64, 255f64, 0f64, -1f64), THICKNESS, LINE_TYPE, SHIFT)?;
            }
        }

        // let mut new_frame = Mat::default()?;
        // frame.convert_to(&mut new_frame, -1, 1.0, 2.0).unwrap();

        highgui::imshow(WINDOW_NAME, &oldframe).unwrap();
        highgui::wait_key(16);
        // highgui::wait_key(((1000.0 / vid.frame_rate) / 4.0) as i32).unwrap();
    
        // let blob = dnn::blob_from_image(&frame, 1.0, Size::new(vid_width as i32, vid_height as i32), Scalar::from(0.007843), false, false, CV_32F).unwrap();
        // // dnet.set_inputs_names(&VectorOfString::from(vec!["test"])).unwrap();
        // dnet.set_input(&blob, "", 1.0, Scalar::from(0.007843)).unwrap();
    
        // let mut new_blobs = Mat::default()?;
        // let names = VectorOfString::from(vec!["person"]);

        // let res = match dnet.forward(&mut new_blobs, &names) {
        //     Ok(res) => println!("{:?}", names),
        //     Err(res) => println!("{}", res),
        // };
    }
}
