use self_update::self_replace;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Checking for updates...");

    let releases = self_update::backends::github::ReleaseList::configure()
        .repo_owner("caglacelik")
        .repo_name("simple_calculator")
        .build()?
        .fetch()?;
    {
        println!("found releases:");
        println!("{:#?}\n", releases);

        // get the first available release
        let asset = releases[0]
            .asset_for(&self_update::get_target(), None)
            .unwrap();

        let tmp_dir = tempfile::Builder::new()
            .prefix("self_update")
            .tempdir_in(::std::env::current_dir()?)?;

        let tmp_tarball_path = tmp_dir.path().join(&asset.name);
        let tmp_tarball = ::std::fs::File::open(&tmp_tarball_path)?;

        self_update::Download::from_url(&asset.download_url)
            .set_header(reqwest::header::ACCEPT, "application/octet-stream".parse()?)
            .download_to(&tmp_tarball)?;

        let bin_name = std::path::PathBuf::from("self_update_bin");

        self_update::Extract::from_source(&tmp_tarball_path)
            .archive(self_update::ArchiveKind::Tar(Some(
                self_update::Compression::Gz,
            )))
            .extract_file(&tmp_dir.path(), &bin_name)?;

        let new_exe = tmp_dir.path().join(bin_name);
        self_replace::self_replace(new_exe)?;
    }

    run_calculator();
    Ok(())
}

fn run_calculator() {
    println!("Welcome to the simple calculator!");
    println!("Enter two numbers:");

    let mut input1 = String::new();
    let mut input2 = String::new();

    io::stdin().read_line(&mut input1).unwrap();
    io::stdin().read_line(&mut input2).unwrap();

    let num1: f64 = input1.trim().parse().unwrap();
    let num2: f64 = input2.trim().parse().unwrap();

    println!("Choose an operation: + or -");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).unwrap();

    let result = match operation.trim() {
        "+" => num1 + num2,
        "-" => num1 - num2,
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    println!("The result is: {}", result);
}
