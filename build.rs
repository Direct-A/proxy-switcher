use embed_resource;

fn main() {
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rerun-if-changed=icon.rc");
        println!("cargo:rerun-if-changed=tray-icon.ico");

        embed_resource::compile("icon.rc", embed_resource::NONE);
    }
}
