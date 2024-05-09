
mod support;

fn main() {
    // read the command Line arguments
    // and prase the arguments to get platform, win, WIN, window, windows or linux
    let args: Vec<String> = std::env::args().collect();
    //if the argument is not provided then print the message and stop the program
    if args.len() < 2 {
        println!("Please provide the platform as argument");
        println!("sush as \"{} [window|linux]\"", &args[0]);
        //exit the program
        return;
    }

    let argument = &args[1].to_uppercase();
    //check the argument
    // if it is equal to WIN, WINDOW, WINDOWS then call the function launch_vlc_win
    // if it is equal to LIN, LINUX then call the function launch_vlc_linux
    if argument == "WIN" || argument == "WINDOWS" || argument == "WINDOW"{
        let video_file = "C:\\Users\\ssuki\\rust-demo\\bigbuckbunny_480.mp4";
        support::launch_vlc_win(video_file);
    } else { //if the argument is not equal to WIN, WINDOW, WINDOWS then call the function launch_vlc_linux
        let video_file = "/home/ssukijth/dev/hls/bigbuckbunny_480.mp4";
          support::launch_vlc_linux(video_file);
    }    
}
