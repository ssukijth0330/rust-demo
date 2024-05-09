//function to launch vlc with the given file
pub fn launch_vlc_linux(file: &str){
     println!("Launching VLC with file: {}", file);
     //call bash command to launch vlc
        std::process::Command::new("cvlc")
            .arg("--fullscreen")
            .arg(file)
            .spawn()
            .expect("VLC failed to start");
    //Note:
    // Up arrow key increase the volume
    // Down arrow key decrease the volume
    // Right arrow key forward the video
}

//function to Launch vlc with a file in windows
pub fn launch_vlc_win(file: &str) {
    //use std::process to launch vlc with flag and a video file
    let _output = std::process::Command::new("C:\\Program Files (x86)\\VideoLAN\\VLC\\vlc.exe") //path to vlc
        .arg("--fullscreen") //flag to remove embedded video
        .arg("--play-and-exit") //flag to play and exit
        .arg("--no-video-title-show") //flag to remove video title
        .arg("--no-qt-error-dialogs") //flag to remove qt error dialogs
        .arg("--no-qt-name-in-title") //flag to remove qt name in title
        .arg(file)
        .output()
        .expect("failed to execute process");
    //Note:
    // Up arrow key increase the volume
    // Down arrow key decrease the volume
    // Right arrow key forward the video
}
