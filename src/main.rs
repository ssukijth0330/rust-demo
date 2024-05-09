
//include the file common.rs
//mod common;
mod support;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let argumet = &args[1];
    if argumet == "win" || argumet == "WIN" || argumet == "Win" || argumet == "windows" || argumet == "WINDOWS" || argumet == "Windows"{
        support::launch_vlc_win("bigbuckbunny_480.mp4");
    } else {
          support::launch_vlc_linux("/home/ssukijth/dev/hls/bigbuckbunny_480.mp4");
    }
    //call the function to launch vlc
}
