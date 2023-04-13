use opencv::{prelude::*, videoio, highgui};

fn main() -> opencv::Result<()> {
    let window = "camera";
    highgui::named_window(window, highgui::WINDOW_NORMAL)?;
    
    let mut cap = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;  // 0 is the default camera
    if !cap.is_opened()? {
        println!("Unable to open camera.");
        return Ok(());
    }
    
    let mut frame = Mat::default();
    loop {
        cap.read(&mut frame)?;
        if frame.size()?.width > 0 {
            highgui::imshow(window, &frame)?;
        } else {
            println!("Failed to capture frame.");
        }
        
        let key = highgui::wait_key(1)?;
        if key > 0 && key != 255 {
            break;
        }
    }

    Ok(())
}


