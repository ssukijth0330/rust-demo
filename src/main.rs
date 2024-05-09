use std::vec;


//include the file common.rs
mod common;
mod support;

fn main() {
    //call the function to launch vlc
    support::launch_vlc("/home/ssukijth/dev/hls/bigbuckbunny_480.mp4");
    // sleep for 5 seconds
    std::thread::sleep(std::time::Duration::from_secs(5));
    //doing fast forward
    support::fast_forward();
    // sleep for 5 seconds
    std::thread::sleep(std::time::Duration::from_secs(5));


}

fn mathmatic(){
    let minimum = common::find_max(vec![1, 2, 3, 104, 5, 6, 7, 8, 9, 10]);
    println!("The maximum number is: {}", minimum);
}