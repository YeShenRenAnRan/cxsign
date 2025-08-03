// Copyright (C) 2025 worksoup <https://github.com/worksoup/>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

mod ui;
use cxlib::{
    types::{DefaultLoginSolver, UntypedLoginSolver},
    AccountCmdApp, AccountsCmdApp, AppInfo, AppTrait, CmdApp, CmdAppContext, ConfigDir,
    CoursesCmdApp, GlobalMultimap, LocationCmdApp, LocationsCmdApp, SignMainApp,
    WhereIsConfigCmdApp,
};

pub fn run() {
    fn init(self_: &CmdApp<CmdAppContext>) -> (CmdAppContext, ()) {
        let env = env_logger::Env::default().filter_or("RUST_LOG", "info");
        let mut builder = env_logger::Builder::from_env(env);
        builder.target(env_logger::Target::Stderr);
        builder.init();
        let app_info = AppInfo::new(
            "TEST_CXSIGN",
            "up.workso",
            "Worksoup",
            env!("CARGO_PKG_NAME"),
        );
        let dir = ConfigDir::new(&app_info);
        let database_file_name = app_info.application();
        let solvers = GlobalMultimap::default();
        solvers.register_builder(|| UntypedLoginSolver::from_typed(DefaultLoginSolver::default()));
        (
            CmdAppContext::new(
                dir,
                database_file_name,
                self_.command().clone(),
                solvers,
                app_info,
            ),
            (),
        )
    }
    let cmd_app = CmdApp::<CmdAppContext>::new(clap::command!())
        .main_app::<SignMainApp>(Default::default())
        .meta_app::<AccountCmdApp>()
        .meta_app::<AccountsCmdApp>()
        .meta_app::<CoursesCmdApp>()
        .meta_app::<LocationCmdApp>()
        .meta_app::<LocationsCmdApp>()
        .meta_app::<WhereIsConfigCmdApp>();
    #[cfg(feature = "completion")]
    let cmd_app = cmd_app.meta_app::<cxlib::CompletionCmdApp>();
    cmd_app.init_and_run(init)
}

#[cfg(test)]
mod tests {
    use crate::ui::NotifyBarTest;
    use slint::ComponentHandle;

    #[test]
    fn test_nofitier() {
        let window = NotifyBarTest::new().unwrap();
        window.run().unwrap();
    }
}
