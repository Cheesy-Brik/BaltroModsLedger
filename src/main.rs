use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct Repo {
    name: String,
    full_name: String,
    description: Option<String>,
    stargazers_count: u32,
}

#[derive(Debug, Serialize)]
pub struct RepoData {
    url: String,
    name: String,
    full_name: String,
    description: Option<String>,
    stars: u32,
}

fn load_github_token() -> String {
    fs::read_to_string(".github_token")
        .context("Failed to read GitHub token from .github_token file")
        .unwrap()
        .trim()
        .to_string()
}

pub async fn fetch_repo_data(repo_url: &str, token: &str) -> Result<RepoData> {
    let client = reqwest::Client::builder()
        .user_agent("rust-github-fetcher")
        .build()
        .context("Failed to create HTTP client")?;

    let repo_info: Repo = client
        .get(
            repo_url
                .trim()
                .replace("github.com", "api.github.com/repos"),
        )
        .header("Accept", "application/vnd.github.v3+json")
        .header("Authorization", format!("token {}", token))
        .send()
        .await
        .context(format!("Failed to fetch repository data for {}", repo_url))?
        .json()
        .await
        .context(format!("Failed to parse repo JSON for {}", repo_url))?;

    Ok(RepoData {
        url: repo_url.to_string(),
        name: repo_info.name,
        full_name: repo_info.full_name,
        description: repo_info.description,
        stars: repo_info.stargazers_count,
    })
}

fn write_readme(repo_data: &[RepoData]) -> Result<()> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("README.md");

    let mut file = fs::File::create(path).context("Failed to create README.md file")?;

    writeln!(
        file,
"# Balatro Mod Ledger
This is a Balatro Mod Ledger to collect Balatro mods into one place. The mod list does not update automatically.
You can request to remove/add mods through this [form](https://docs.google.com/forms/d/e/1FAIpQLScMxnOOnqRa7eklt6hEq_FDpHj7MXcnP6QzZFxoOwRvQO4aRA/viewform?usp=sharing).
## Mods - (*{}* in total)
(*sorted by github stars*)", repo_data.len()
    )
    .expect("Error in writing to file");

    for repo in repo_data {
        writeln!(
            file,
            "1. **[{}]({})** - {} ⭐{}",
            repo.name,
            repo.url,
            repo.description.clone().unwrap_or("".to_string()),
            repo.stars
        )
        .expect("Error in writing to file");
    }

    writeln!(
        file,
        "## *PS*
Please give your favorite mods github stars. It helps with Google results and is a good motivator."
    )
    .expect("Error in writing to file");

    Ok(())
}

const REPOS: &str = "
https://github.com/MathIsFun0/Cryptid
https://github.com/besteon/balatrobot
https://github.com/Numbuh214/EnhanceAPI
https://github.com/itayfeder/FederAPI-Balatro
https://github.com/AutumnMood924/OddityAPI
https://github.com/Infarcactus/Balatro-Custom-Sound-Player
https://github.com/Bazinga9000/MathBlinds
https://github.com/betmma/my_balatro_mods
https://github.com/morpline/FeedTheVampire
https://github.com/UppedHealer8521/Riff-Raffling
https://github.com/Aurelius7309/StylishSleeves
https://github.com/Tucaonormal/LastStand
https://github.com/ascriptmaster/Balatro-Nihility
https://github.com/nicholassam6425/balatro-mods
https://github.com/Mysthaps/BalatroMods
https://github.com/Wiwiweb/BalatroMods
https://github.com/itayfeder/Codex-Arcanum
https://github.com/JeffVi/DX-Tarots
https://github.com/encarvlucas/EncarvlucasBalatroMods
https://github.com/GuilloryCraft/ExtraCredit
https://github.com/Firch/Bunco
https://github.com/Minirebel/Jokebook
https://github.com/Dimserene/ModpackManager
https://github.com/lshtech/morjokers
https://github.com/lshtech/Matchbox
https://github.com/lshtech/LushMod
https://github.com/lshtech/balatro-pampa-joker-pack
https://github.com/lshtech/Balatro-HandPreview
https://github.com/lshtech/DiscoveryManager
https://github.com/lshtech/CardExporter
https://github.com/MrSmoothieHuman1/MOARJokers
https://github.com/art-muncher/Jimbo-s-Pack
https://github.com/LunaAstraCassiopeia/FusionForce
https://github.com/SDM0/SDM_0-s-Stuff
https://github.com/Mathguy23/Grim
https://github.com/SleepyG11/TwitchBlinds
https://github.com/Mysthaps/LobotomyCorp
https://github.com/LunaAstraCassiopeia/DreadJokers
https://github.com/itayfeder/Fusion-Jokers
https://github.com/Eremel/Ortalab
https://github.com/jenwalter666/JensBalatroCollection
https://github.com/InertSteak/Pokermon
https://github.com/MathIsFun0/Talisman
https://github.com/0fficialHalo/Gemstones
https://github.com/stupxd/Cartomancer
https://github.com/notmario/MoreFluff
https://github.com/Eremel/Galdur
https://github.com/KaviD-115/Balatro-Jokers-Plus
https://github.com/nekojoe/Ceres
https://github.com/Maratby/Tsunami
https://github.com/parchmentEngineer/The-World-Ends-With-Jimbo
https://github.com/wingedcatgirl/MintysSillyMod
https://github.com/stupxd/Blueprint
https://github.com/art-muncher/Card-Value-Display
https://github.com/GitNether/paperback
https://github.com/spikeof2010/JankJonklers
https://github.com/Mysthaps/LobotomyCorp/
https://github.com/Minirebel/no-laughing-matter
https://github.com/batabata3/balatro-pampa-joker-pack
https://github.com/RattlingSnow353/Snow-s-Mods
https://github.com/RitchieDimaria/SpicyJokers
https://github.com/DigitalDetective47/strange-pencil
https://github.com/jrings/circus
https://github.com/BlizzowX/Balatro---Themed-Jokers
https://github.com/DigitalDetective47/yart
https://github.com/KilledByLava/BossJokers
https://github.com/pinkmaggit-hub/Buffoonery
https://github.com/BarrierTrio/Cardsauce
https://github.com/larswijn/CardSleeves
https://github.com/ilikecheese0/CheesyJokers
https://github.com/RattlingSnow353/Familiar
https://github.com/kcgidw/kcvanilla
https://github.com/AvilionAMillion/Komakusa-Cards
https://github.com/snowylight/Multipack
https://github.com/Steamopollys/Steamodded
https://github.com/MathIsFun0/Aura
https://github.com/MathIsFun0/Trance
";

#[tokio::main]
async fn main() -> Result<()> {
    let repo_urls: Vec<String> = REPOS
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|url| url.to_string())
        .collect();

    let token = load_github_token();

    let futures: Vec<_> = repo_urls
        .iter()
        .map(|url| fetch_repo_data(&url, &token))
        .collect();

    let results = futures::future::join_all(futures).await;

    let mut repo_data: Vec<RepoData> = results
        .into_iter()
        .filter_map(|result| match result {
            Ok(data) => Some(data),
            Err(e) => {
                eprintln!("Error fetching repo data: {}", e);
                None
            }
        })
        .collect();
    repo_data.sort_by(|a, b| a.name.cmp(&b.name));
    repo_data.sort_by(|a, b| b.stars.cmp(&a.stars));

    write_readme(&repo_data)?;

    Ok(())
}
