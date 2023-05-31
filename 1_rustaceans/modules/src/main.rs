mod player;

fn main() {
    player::play_movie("The Matrix");
    player::play_audio("The Beatles");

    clean::perform_clean();
    clean::files::clean_files();
}

mod clean {
    pub fn perform_clean() {
        println!("Cleaning the house");
    }

    pub mod files {
        pub fn clean_files() {
            println!("Cleaning files");
        }
    }
}
