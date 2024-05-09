//function to launch vlc with the given file
pub fn launch_vlc_linux(file: &str){
     println!("Launching VLC with file: {}", file);
     //call bash command to launch vlc
        std::process::Command::new("cvlc")
            .arg("--fullscreen")
            .arg(file)
            .spawn()
            .expect("VLC failed to start");
}

//function to Launch vlc with a file in windows
pub fn launch_vlc_win(file: &str) {
    let cmd = "-no-video-deco";
    //use std::process to launch vlc with flag and a video file
    let output = std::process::Command::new("C:\\Program Files (x86)\\VideoLAN\\VLC\\vlc.exe") //path to vlc
        .arg("--fullscreen") //flag to remove embedded video
        .arg("--play-and-exit") //flag to play and exit
        .arg("--no-video-title-show") //flag to remove video title
        .arg("--no-qt-error-dialogs") //flag to remove qt error dialogs
        .arg("--no-qt-name-in-title") //flag to remove qt name in title
        .arg(file)
        .output()
        .expect("failed to execute process");
    //print the output
    println!("{:?}", output);
}

// //function to fast forward the video in vlc
// pub fn fast_forward(){
//     println!("Fast forwarding video");
//     //call bash command to fast forward the video
//     std::process::Command::new("xdotool")
//         .arg("key")
//         .arg("Right")
//         .spawn()
//         .expect("Fast forward failed");
// }

// //function to rewind the video in vlc
// pub fn rewind(){
//     println!("Rewinding video");
//     //call bash command to rewind the video
//     std::process::Command::new("xdotool")
//         .arg("key")
//         .arg("Left")
//         .spawn()
//         .expect("Rewind failed");
// }
// //function to pause the video in vlc
// pub fn pause(){
//     println!("Pausing video");
//     //call bash command to pause the video
//     std::process::Command::new("xdotool")
//         .arg("key")
//         .arg("space")
//         .spawn()
//         .expect("Pause failed");
// }
// //function to stop the video in vlc
// pub fn stop(){
//     println!("Stopping video");
//     //call bash command to stop the video
//     std::process::Command::new("xdotool")
//         .arg("key")
//         .arg("s")
//         .spawn()
//         .expect("Stop failed");
// }
// //function to play the video in vlc
// pub fn play(){
//     println!("Playing video");
//     //call bash command to play the video
//     std::process::Command::new("xdotool")
//         .arg("key")
//         .arg("p")
//         .spawn()
//         .expect("Play failed");
// }
// //function to increase the volume of the video in vlc
// pub fn volume_up(){
//     println!("Increasing volume");
//     //call bash command to increase the volume
//     std::process::Command::new("xdotool")
//         .arg("key")
//         .arg("Up")
//         .spawn()
//         .expect("Volume up failed");
// }
// //function to decrease the volume of the video in vlc
// pub fn volume_down(){
//     println!("Decreasing volume");
//     //call bash command to decrease the volume
//     std::process::Command::new("xdotool")
//         .arg("key")
//         .arg("Down")
//         .spawn()
//         .expect("Volume down failed");
// }
// //function to mute the video in vlc
// pub fn mute(){
//     println!("Muting video");
//     //call bash command to mute the video
//     std::process::Command::new("xdotool")
//         .arg("key")
//         .arg("m")
//         .spawn()
//         .expect("Mute failed");
// }
// //function to take a screenshot of the video in vlc
// pub fn screenshot(){
//     println!("Taking screenshot");
//     //call bash command to take a screenshot
//     std::process::Command::new("xdotool")
//         .arg("key")
//         .arg("s")
//         .spawn()
//         .expect("Screenshot failed");
// }
// //function to toggle fullscreen of the video in vlc
// pub fn fullscreen(){
//     println!("Toggling fullscreen");
//     //call bash command to toggle fullscreen
//     std::process::Command::new("xdotool")
//         .arg("key")
//         .arg("f")
//         .spawn()
//         .expect("Fullscreen failed");
// }
// //function to toggle loop of the video in vlc
// pub fn loop_video(){
//     println!("Toggling loop");
//     //call bash command to toggle loop
//     std::process::Command::new("xdotool")
//         .arg("key")
//         .arg("l")
//         .spawn()
//         .expect("Loop failed");
// }
// //function to toggle random of the video in vlc
// pub fn random(){
//     println!("Toggling random");
//     //call bash command to toggle random
//     std::process::Command::new("xdotool")
//         .arg("key")
//         .arg("z")
//         .spawn()
//         .expect("Random failed");
// }
// //function to toggle repeat of the video in vlc
// pub fn repeat(){
//     println!("Toggling repeat");
//     //call bash command to toggle repeat
//     std::process::Command::new("xdotool")
//         .arg("key")
//         .arg("r")
//         .spawn()
//         .expect("Repeat failed");
// }
// //function to toggle shuffle of the video in vlc
// pub fn shuffle(){
//     println!("Toggling shuffle");
//     //call bash command to toggle shuffle
//     std::process::Command::new("xdotool")
//         .arg("key")
//         .arg("b")
//         .spawn()
//         .expect("Shuffle failed");
// }
// //function to toggle the equalizer of the video in vlc
// pub fn equalizer(){
//     println!("Toggling equalizer");
//     //call bash command to toggle equalizer
//     std::process::Command::new("xdotool")
//         .arg("key")
//         .arg("e")
//         .spawn()
//         .expect("Equalizer failed");
// }
// //function for single key event
// pub fn key_event(key: &str){
//     println!("Key event: {}", key);
//     //call bash command for key event
//     std::process::Command::new("xdotool")
//         .arg("key")
//         .arg(key)
//         .spawn()
//         .expect("Key event failed");
// }
//function quit vlc
// pub fn quit(){
//     println!("Quitting VLC");
//     //call bash command to quit vlc
//     std::process::Command::new("xdotool")
//         .arg("key")
//         .arg("q")
//         .spawn()
//         .expect("Quit failed");
// }
