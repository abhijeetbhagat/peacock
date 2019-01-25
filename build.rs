fn main(){
    println!("cargo:rustc-link-lib=static=gstreamer-1.0");
    println!(r"cargo:rustc-link-search=c:\gstreamer\1.0\x86_64\lib");
    println!(r"cargo:rustc-link-search=c:\gstreamer\1.0\x86_64\bin");
}
