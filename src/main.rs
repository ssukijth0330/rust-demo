
//include the file common.rs
//mod common;
mod support;

fn main() {
    // read the command Line arguments
    // and prase the arguments to get platform, win, WIN, window, windows or linux
    let args: Vec<String> = std::env::args().collect();
    let argumet = &args[1].to_uppercase();
    //check the argument
    // if it is equal to WIN, WINDOW, WINDOWS then call the function launch_vlc_win
    // if it is equal to LIN, LINUX then call the function launch_vlc_linux
    if argumet == "WIN" || argumet == "WINDOWS" || argumet == "WINDOW"{
        support::launch_vlc_win("bigbuckbunny_480.mp4");
    } else {
          support::launch_vlc_linux("/home/ssukijth/dev/hls/bigbuckbunny_480.mp4");
    }
    //call the function to launch vlc
}
