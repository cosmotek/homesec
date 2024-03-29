// extern crate opencv;
// use opencv::highgui;
// use std::thread;
// use std::time::{Duration, Instant};
// use std::env;
// use std::fs;

// // use opencv::{
// // 	highgui,
// // 	prelude::*,
// // 	Result,
// // 	videoio,
// // };

// use opencv::videoio::{VideoCapture, CAP_ANY, CAP_FFMPEG, CAP_PROP_FPS, CAP_PROP_FRAME_COUNT, CAP_PROP_FRAME_HEIGHT, CAP_PROP_FRAME_WIDTH, CAP_PROP_POS_FRAMES};
// use opencv::videoio::prelude::VideoCaptureTrait;
// use opencv::core::{Mat, Scalar, Size, Vector, Point};
// // use opencv::types::{VectorOfString};
// // use opencv::imgproc::{cvt_color, COLOR_RGB2RGBA};
// use opencv::imgcodecs::{imwrite, IMWRITE_JPEG_QUALITY};
// // use opencv::core::prelude::MatTrait;
// // use opencv::objdetect::{CascadeClassifier};
// // use opencv::objdetect::CascadeClassifierTrait;
// use opencv::core::{Rect};
// use opencv::{imgproc, objdetect, prelude::*, types};



// // fn read_config() {
// //     let deserialized_point: Point = serde_yaml::from_str(&s)?;
// // }


// const WINDOW_NAME: &str = "Optimus Engine Visual Debugger";

// // // https://www.pyimagesearch.com/2019/09/02/opencv-stream-video-to-web-browser-html-page/

// const CAPTURE_WIDTH: i32 = 800;
// const CAPTURE_HEIGHT: i32 = 600;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // let input_video_file = "/home/cosmotek/Downloads/The.Unicorn.S01E12.HDTV.x264-KILLERS[ettv]/The.Unicorn.S01E12.HDTV.x264-KILLERS[ettv].mkv";
//     let classifer_file = "/Users/seth.moeckel/code/rust/homesec/haarcascade_frontalface_default.xml";
//     // let input_video_file = "rtsp://192.168.0.21:5554";

//     // env::set_var("OPENCV_FFMPEG_CAPTURE_OPTIONS", "rtsp_transport;udp");
//     // let mut vid = VideoCapture::from_file(input_video_file, CAP_FFMPEG).unwrap();
//     let mut vid = VideoCapture::new(0, CAP_ANY)?;
//     while !vid.is_opened()? {
//         println!("awaiting video open...");
//         thread::sleep(Duration::from_millis(1000));
//     }

//     println!("starting feed");
//     fs::create_dir_all("clippings")?;
    
//     // vid.set(CAP_PROP_POS_FRAMES, 4000.0);
//     let mut class = objdetect::CascadeClassifier::new(classifer_file)?;

//     highgui::named_window(WINDOW_NAME, highgui::WINDOW_AUTOSIZE).unwrap();
//     // highgui::resize_window(WINDOW_NAME, 800, 600).unwrap();

//     let mut imgs = 0;
//     let mut a_time = Instant::now();
//     let mut b_time = Instant::now();

//     loop {
//         a_time = b_time;
//         b_time = Instant::now();
//         let delta_time = b_time.duration_since(a_time);

//         let mut oldframe = Mat::default();
//         let next_exists = vid.read(&mut oldframe).unwrap();
//         // let mut frame = Mat::default()?;
//         // imgproc::cvt_color(&mut oldframe, &mut frame, imgproc::COLOR_BGR2GRAY, 0)?;

//         if next_exists {
//             const SCALE_FACTOR: f64 = 1.2;
//             const SCALE_FACTOR_INV: i32 = (1f64 / SCALE_FACTOR) as i32;

//             const MIN_NEIGHBORS: i32 = 2;
//             const FLAGS: i32 = 0;
//             const MIN_FACE_SIZE: Size = Size {
//                 width: 10,
//                 height: 10,
//             };
//             const MAX_FACE_SIZE: Size = Size {
//                 width: 250,
//                 height: 250,
//             };

//             let mut faces = types::VectorOfRect::new();
//             class.detect_multi_scale(
//                 &oldframe,
//                 &mut faces,
//                 SCALE_FACTOR,
//                 MIN_NEIGHBORS,
//                 FLAGS,
//                 MIN_FACE_SIZE,
//                 MAX_FACE_SIZE,
//             )?;

//             // println!("{} faces", faces.len());
//             for face in faces {
//                 // println!("{}x{} [{}, {}]", face.x, face.y, face.width, face.height);
//                 let scaled_face = Rect {
//                     x: (face.x - 100) * SCALE_FACTOR_INV,
//                     y: (face.y - 100) * SCALE_FACTOR_INV,
//                     width: face.width * SCALE_FACTOR_INV,
//                     height: face.height * SCALE_FACTOR_INV,
//                 };

//                 const THICKNESS: i32 = 2;
//                 const LINE_TYPE: i32 = 8;
//                 const SHIFT: i32 = 0;

//                 let face_clip = Mat::roi(&oldframe.clone(), face)?;
//                 imgs += 1;

//                 let mut compression_params = Vector::new();
//                 compression_params.push(IMWRITE_JPEG_QUALITY);
//                 compression_params.push(100);

//                 thread::spawn(move || {
//                     imwrite(&format!("clippings/clipping{}.jpg", imgs)[..], &face_clip, &compression_params).unwrap();
//                 });
            
//                 // imgproc::cvt_color(&mut frame, &mut oldframe, imgproc::COLOR_GRAY2BGR, 0)?;
//                 imgproc::rectangle(&mut oldframe, face, Scalar::new(86f64, 220f64, 254f64, -1f64), THICKNESS, LINE_TYPE, SHIFT)?;
//                 imgproc::put_text(&mut oldframe, "unknown", Point::new(face.x, face.y-10), imgproc::FONT_HERSHEY_PLAIN, 1.0, Scalar::new(86f64, 220f64, 254f64, -1f64), 1, LINE_TYPE, false)?;
//             }

//             let mut delta_millis = delta_time.as_millis();
//             if delta_millis == 0 {
//                 delta_millis = 16;
//             }
//             imgproc::put_text(&mut oldframe, &format!("FPS: {}", (1000 / delta_millis))[..], Point::new(5, 20), imgproc::FONT_HERSHEY_PLAIN, 1.5, Scalar::new(0.0, 0.0, 255.0, -1f64), 2, 8, false)?;
//         }

//         // let mut new_frame = Mat::default()?;
//         // frame.convert_to(&mut new_frame, -1, 1.0, 2.0).unwrap();

//         highgui::imshow(WINDOW_NAME, &oldframe).unwrap();
//         highgui::wait_key(1)?;
//         // highgui::wait_key(((1000.0 / vid.frame_rate) / 4.0) as i32).unwrap();
    
//         // let blob = dnn::blob_from_image(&frame, 1.0, Size::new(vid_width as i32, vid_height as i32), Scalar::from(0.007843), false, false, CV_32F).unwrap();
//         // // dnet.set_inputs_names(&VectorOfString::from(vec!["test"])).unwrap();
//         // dnet.set_input(&blob, "", 1.0, Scalar::from(0.007843)).unwrap();
    
//         // let mut new_blobs = Mat::default()?;
//         // let names = VectorOfString::from(vec!["person"]);

//         // let res = match dnet.forward(&mut new_blobs, &names) {
//         //     Ok(res) => println!("{:?}", names),
//         //     Err(res) => println!("{}", res),
//         // };
//     }
// }
// // extern crate image;
// // use image::{RgbImage, Rgb};

// // use dlib_face_recognition::*;

// // fn draw_rectangle(image: &mut RgbImage, rect: &Rectangle, colour: Rgb<u8>) {
// //     for x in rect.left..rect.right {
// //         image.put_pixel(x as u32, rect.top as u32, colour);
// //         image.put_pixel(x as u32, rect.bottom as u32, colour);
// //     }

// //     for y in rect.top..rect.bottom {
// //         image.put_pixel(rect.left as u32, y as u32, colour);
// //         image.put_pixel(rect.right as u32, y as u32, colour);
// //     }
// // }

// // fn draw_point(image: &mut RgbImage, point: &Point, colour: Rgb<u8>) {
// //     image.put_pixel(point.x() as u32, point.y() as u32, colour);
// //     image.put_pixel(point.x() as u32 + 1, point.y() as u32, colour);
// //     image.put_pixel(point.x() as u32 + 1, point.y() as u32 + 1, colour);
// //     image.put_pixel(point.x() as u32, point.y() as u32 + 1, colour);
// // }

// // fn tick<R>(name: &str, f: impl Fn() -> R) -> R {
// //     let now = std::time::Instant::now();
// //     let result = f();
// //     println!("[{}] elapsed time: {}ms", name, now.elapsed().as_millis());
// //     result
// // }

// // #[cfg(feature = "embed-all")]
// // fn main() {
// //     let mut args = std::env::args().skip(1);
// //     let input = args.next().unwrap();
// //     let output = args.next().unwrap();

// //     let mut image = image::open(input).unwrap().to_rgb8();
// //     let matrix = ImageMatrix::from_image(&image);

// //     let detector = FaceDetector::new();
// //     let cnn_detector = FaceDetectorCnn::new("/home/cosmotek/code/rust/homesec/models/shape_predictor_68_face_landmarks.dat").unwrap();
// //     let landmarks = LandmarkPredictor::new("/home/cosmotek/code/rust/homesec/models/mmod_human_face_detector.dat").unwrap();
// //     // let landmarks = LandmarkPredictor::default();

// //     let red = Rgb([255, 0, 0]);
// //     let green = Rgb([0, 255, 0]);

// //     let face_locations = tick("FaceDetector", || detector.face_locations(&matrix));

// //     for r in face_locations.iter() {
// //         draw_rectangle(&mut image, &r, red);

// //         let landmarks = landmarks.face_landmarks(&matrix, &r);

// //         for point in landmarks.iter() {
// //             draw_point(&mut image, &point, red);
// //         }
// //     }

// //     let face_locations = tick("FaceDetectorCnn", || cnn_detector.face_locations(&matrix));

// //     for r in face_locations.iter() {
// //         draw_rectangle(&mut image, &r, green);

// //         let landmarks = tick("LandmarkPredictor", || {
// //             landmarks.face_landmarks(&matrix, &r)
// //         });

// //         for point in landmarks.iter() {
// //             draw_point(&mut image, &point, green);
// //         }
// //     }

// //     image.save(&output).unwrap();
// // }
// // extern crate serde_yaml;
// // extern crate toml;

// // #[macro_use]
// // extern crate serde_derive;

// // // use serde_yaml::{Error};
// // use serde::{Serialize, Deserialize};

// // #[derive(Debug, PartialEq, Serialize, Deserialize)]
// // struct XYSize {
// //     width: i32,
// //     height: i32,
// // }

// // #[derive(Debug, PartialEq, Serialize, Deserialize)]
// // struct FaceDetectionParams {
// //     min_neighbors: i32,
// //     min_face_size: XYSize,
// //     max_face_size: XYSize,
// // }

// // #[derive(Debug, PartialEq, Serialize, Deserialize)]
// // struct Config {
// //     rtsp_devices: Vec<String>,
// //     // detection_params: FaceDetectionParams,
// //     data_dir: String,
// // }

// // #[cfg(not(feature = "embed-all"))]
// // fn main() {
// //     panic!("You need to run this example with '--features embed-all'.");
// //     // let conf = Config {
// //     //     rtsp_devices: ["192.168.0.21:5554".to_string()].to_vec(),
// //     //     data_dir: "homesec/data/".to_string(),
// //     //     // detection_params: FaceDetectionParams {
// //     //     //     min_neighbors: 2,
// //     //     //     min_face_size: XYSize {
// //     //     //         width: 10,
// //     //     //         height: 10,
// //     //     //     },
// //     //     //     max_face_size: XYSize {
// //     //     //         width: 250,
// //     //     //         height: 250,
// //     //     //     },
// //     //     // },
// //     // };

// //     // let s = toml::to_string(&conf).unwrap();
// //     // println!("{}", s);
// // }


// // use std::env;
// // mod notify;
// // use notify::{Notifier};

// // #[tokio::main]
// // async fn main() {
// //     let app_id = env::var("TWILIO_APP_ID").unwrap();
// //     let auth_token = env::var("TWILIO_AUTH_TOKEN").unwrap();

// //     let notifier = notify::new(
// //         app_id,
// //         auth_token,
// //         "+16318372032".to_string(),
// //         ["+17408772320".to_string()].to_vec(),
// //     );

// //     Notifier::notify_entry(&notifier, [
// //         "Salvi".to_string(),
// //         "1238798232r".to_string(),
// //         "James".to_string(),
// //     ].to_vec()).await;
// // }

use std::{thread, time::Duration};

use opencv::{
	core,
	highgui,
	imgproc,
	objdetect,
	prelude::*,
	Result,
	types,
	videoio,
};

fn main() -> Result<()> {
	let window = "video capture";
    let cascade_uri = "/Users/seth.moeckel/code/rust/homesec/haarcascade_frontalface_default.xml";
	highgui::named_window(window, 1)?;
	#[cfg(ocvrs_opencv_branch_32)]
	let (xml, mut cam) = {
		(
			cascade_uri.to_owned(),
			videoio::VideoCapture::new_default(0)?, // 0 is the default camera
		)
	};
	#[cfg(not(ocvrs_opencv_branch_32))]
	let (xml, mut cam) = {
		(
			core::find_file(cascade_uri, true, false)?,
			videoio::VideoCapture::new(0, videoio::CAP_ANY)?, // 0 is the default camera
		)
	};
	let opened = videoio::VideoCapture::is_opened(&cam)?;
	if !opened {
		panic!("Unable to open default camera!");
	}
	let mut face = objdetect::CascadeClassifier::new(&xml)?;
	loop {
		let mut frame = Mat::default();
		cam.read(&mut frame)?;
		if frame.size()?.width == 0 {
			thread::sleep(Duration::from_secs(50));
			continue;
		}
		let mut gray = Mat::default();
		imgproc::cvt_color(
			&frame,
			&mut gray,
			imgproc::COLOR_BGR2GRAY,
			0,
		)?;
		let mut reduced = Mat::default();
		imgproc::resize(
			&gray,
			&mut reduced,
			core::Size {
				width: 0,
				height: 0,
			},
			0.25f64,
			0.25f64,
			imgproc::INTER_LINEAR,
		)?;
		let mut faces = types::VectorOfRect::new();
		face.detect_multi_scale(
			&reduced,
			&mut faces,
			1.1,
			2,
			objdetect::CASCADE_SCALE_IMAGE,
			core::Size {
				width: 30,
				height: 30,
			},
			core::Size {
				width: 0,
				height: 0,
			},
		)?;
		println!("faces: {}", faces.len());
		for face in faces {
			println!("face {:?}", face);
			let scaled_face = core::Rect {
				x: face.x * 4,
				y: face.y * 4,
				width: face.width * 4,
				height: face.height * 4,
			};
			imgproc::rectangle(
				&mut frame,
				scaled_face,
				core::Scalar::new(86f64, 220f64, 254f64, -1f64),
				1,
				8,
				0,
			)?;

            imgproc::put_text(
                &mut frame, "unknown",
                core::Point {
                    x: face.x * 4,
				    y: (face.y * 4) - 10,
                },
                imgproc::FONT_HERSHEY_PLAIN,
                1.0,
                core::Scalar::new(86f64, 220f64, 254f64, -1f64),
                1,
                8,
                false,
            )?;
		}
		highgui::imshow(window, &frame)?;
		if highgui::wait_key(10)? > 0 {
			break;
		}
	}
	Ok(())
}