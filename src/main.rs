mod astar;
mod astar_app;
mod testsuite;
mod utils;
use astar::mlcs_astar;

fn main() {
    let chains = vec![
        "🤶🤶99🤶🤶🤶7🤶7🤶🤶🤶6ghg",
        "字字9字字字9776字字字ghg字",
        "据据9据据据97据7据据6gh据g",
        "l9llll9l776lglhgl",
        "p9pp9p7p7pp6pghpg",
        "Q9Q9QQ7Q76QQQghQg",
        "术术99术7术7术术6术术g术hg",
    ];

    let res = mlcs_astar(&chains);

    println!("{}", res);
}
