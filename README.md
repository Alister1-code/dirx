Dirx has one goal. bring bundled applications to Linux, the folders that hold the executable will be named .bundle(for example game.bundle)
which will be ran through dirx, that will then check a couple of thing. it'll ensure that the argument given is a dir and if it ends in .bundle
and once checked it'll read the toml file in the root, this file will let the application know where the executable is and the name of the app
along with a boolean option called "uses_assets" if the app uses assets it'll use the assets folder as it's working directory otherwise it'll
keep the working directory as the executable folder. which will be stored as Content/Linux.

##License
This project is licensed under the GNU General Public License v3.0 or later - see the [LICENSE](LICENSE) file for details.
