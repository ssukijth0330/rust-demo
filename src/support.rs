//function to launch vlc with the given file
pub fn launch_vlc(video_file: &str, vlc_path: &str) {
     println!("Launching VLC with file: {} {}",vlc_path, video_file);
     //call bash command to launch vlc
        std::process::Command::new(vlc_path)
            .arg("--fullscreen")
            .arg("--play-and-exit") //flag to play and exit
            .arg("--no-video-title-show") //flag to remove video title
            .arg("--no-qt-error-dialogs") //flag to remove qt error dialogs
            .arg("--no-qt-name-in-title") //flag to remove qt name in title
                .arg(video_file)
            .spawn()
            .expect("VLC failed to start");
    //Note:
    // Up arrow key increase the volume
    // Down arrow key decrease the volume
    // Right arrow key forward the videogit 
}
